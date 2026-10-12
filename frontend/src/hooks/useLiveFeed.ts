import { useEffect, useRef, useState } from 'react'

/**
 * Where a live feed stands, for the page's indicator:
 * - `connecting` — opening the stream for the first time
 * - `live` — the stream is open and pushing updates
 * - `reconnecting` — the stream dropped; retrying it
 * - `polling` — the stream keeps failing, so the page is polling the REST endpoint instead
 *   (and still retrying the stream every so often)
 * - `error` — refused (not authorized, or not found); not retried
 */
export type LiveStatus = 'connecting' | 'live' | 'reconnecting' | 'polling' | 'error'

export interface LiveFeedOptions<T> {
  /** The stream endpoint, e.g. `/api/tournaments/{tid}/roommonitor/stream`. `null` disables the feed. */
  url: string | null
  /** Sent as `Authorization: Bearer …` on a restricted stream. A new token reconnects the stream. */
  token?: string
  /** The REST read of the same data, used while the stream cannot connect. */
  poll: () => Promise<T>
  /** How often to poll while falling back. */
  pollIntervalMs: number
  /** Called when the server ends the stream because the access token expired. */
  onExpired?: () => void
}

/** Consecutive failed connections before the page starts polling. */
const FAILURES_BEFORE_POLLING = 3
/** Waits between reconnection attempts, by attempt. */
const RETRY_DELAYS_MS = [1_000, 2_000, 5_000]
/** How often to try the stream again while polling. */
const RETRY_WHILE_POLLING_MS = 30_000
/** The server sends a keepalive every 15 s; this long without a byte means the connection is dead. */
const SILENCE_LIMIT_MS = 45_000

/**
 * Subscribes to a backend live-update stream (Server-Sent Events; see backend/src/live.rs).
 *
 * Reads the stream with `fetch` rather than `EventSource`, which cannot send an Authorization
 * header. Every event carries the page's whole data set, so `data` is always simply the latest.
 * The stream is an improvement on polling, never a requirement: if it cannot connect, the hook
 * polls `poll` instead and keeps retrying the stream in the background.
 */
export function useLiveFeed<T>({ url, token, poll, pollIntervalMs, onExpired }: LiveFeedOptions<T>) {
  const [data, setData] = useState<T | null>(null)
  const [status, setStatus] = useState<LiveStatus>('connecting')
  const [error, setError] = useState<string | null>(null)

  // The latest callbacks, read by the running connection without restarting it.
  const pollRef = useRef(poll)
  const onExpiredRef = useRef(onExpired)
  useEffect(() => {
    pollRef.current = poll
    onExpiredRef.current = onExpired
  })

  useEffect(() => {
    if (!url) return
    const stop = new AbortController()
    let failures = 0
    let polling = false
    let pollTimer: number | undefined

    const startPolling = () => {
      if (polling) return
      polling = true
      setStatus('polling')
      const load = () => {
        pollRef.current()
          .then((fetched) => {
            if (stop.signal.aborted) return
            setData(fetched)
            setError(null)
          })
          .catch((e: Error) => {
            if (!stop.signal.aborted) setError(e.message)
          })
      }
      load()
      pollTimer = window.setInterval(load, pollIntervalMs)
    }

    const stopPolling = () => {
      if (!polling) return
      polling = false
      window.clearInterval(pollTimer)
    }

    /** Opens the stream once and reads it until it ends. */
    const connectOnce = async (): Promise<'ended' | 'expired' | 'refused' | 'unavailable'> => {
      const connection = new AbortController()
      const abortConnection = () => connection.abort()
      stop.signal.addEventListener('abort', abortConnection)
      let watchdog: number | undefined
      const heardFromServer = () => {
        window.clearTimeout(watchdog)
        watchdog = window.setTimeout(abortConnection, SILENCE_LIMIT_MS)
      }

      try {
        heardFromServer()
        const res = await fetch(url, {
          headers: {
            Accept: 'text/event-stream',
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
          },
          cache: 'no-store',
          signal: connection.signal,
        })
        if (res.status === 401 || res.status === 403) {
          setError('You are not authorized to view this page.')
          return 'refused'
        }
        if (res.status === 404) {
          setError('Not found.')
          return 'refused'
        }
        // The server has no live hub: poll rather than keep retrying.
        if (res.status === 503) return 'unavailable'
        if (!res.ok || !res.body) return 'ended'

        let expired = false
        await readServerSentEvents(res.body, heardFromServer, (event, payload) => {
          if (event === 'snapshot') {
            setData(JSON.parse(payload) as T)
            setError(null)
            setStatus('live')
            failures = 0
            stopPolling()
          } else if (event === 'expired') {
            expired = true
          }
        })
        return expired ? 'expired' : 'ended'
      } catch {
        return 'ended' // dropped, aborted by the watchdog, or a network error
      } finally {
        window.clearTimeout(watchdog)
        stop.signal.removeEventListener('abort', abortConnection)
      }
    }

    const run = async () => {
      setStatus('connecting')
      while (!stop.signal.aborted) {
        const outcome = await connectOnce()
        if (stop.signal.aborted) return
        if (outcome === 'refused') {
          stopPolling()
          setStatus('error')
          return
        }
        if (outcome === 'expired') {
          // A fresh token re-runs this effect, which replaces this loop.
          onExpiredRef.current?.()
        }
        failures = outcome === 'unavailable' ? FAILURES_BEFORE_POLLING : failures + 1
        if (failures >= FAILURES_BEFORE_POLLING) startPolling()
        else setStatus('reconnecting')

        const delay = polling
          ? RETRY_WHILE_POLLING_MS
          : RETRY_DELAYS_MS[Math.min(failures - 1, RETRY_DELAYS_MS.length - 1)]
        await sleep(delay, stop.signal)
      }
    }

    run()
    return () => {
      stop.abort()
      stopPolling()
    }
  }, [url, token, pollIntervalMs])

  return { data, status, error }
}

/**
 * Reads a `text/event-stream` body, calling `onEvent` once per event with its type and data.
 * Comment lines (the server's keepalives) are skipped but still count as `onBytes`.
 */
async function readServerSentEvents(
  body: ReadableStream<Uint8Array>,
  onBytes: () => void,
  onEvent: (event: string, data: string) => void,
): Promise<void> {
  const reader = body.getReader()
  const decoder = new TextDecoder()
  let buffer = ''
  try {
    for (;;) {
      const { value, done } = await reader.read()
      if (done) return
      onBytes()
      buffer += decoder.decode(value, { stream: true })

      // Events are separated by a blank line.
      let end: number
      while ((end = buffer.indexOf('\n\n')) >= 0) {
        const block = buffer.slice(0, end)
        buffer = buffer.slice(end + 2)

        let event = 'message'
        const dataLines: string[] = []
        for (const rawLine of block.split('\n')) {
          const line = rawLine.endsWith('\r') ? rawLine.slice(0, -1) : rawLine
          if (line === '' || line.startsWith(':')) continue
          const colon = line.indexOf(':')
          const field = colon < 0 ? line : line.slice(0, colon)
          const raw = colon < 0 ? '' : line.slice(colon + 1)
          const fieldValue = raw.startsWith(' ') ? raw.slice(1) : raw
          if (field === 'event') event = fieldValue
          else if (field === 'data') dataLines.push(fieldValue)
        }
        if (dataLines.length > 0) onEvent(event, dataLines.join('\n'))
      }
    }
  } finally {
    reader.releaseLock()
  }
}

/** Waits `ms`, or less if `signal` aborts first. */
function sleep(ms: number, signal: AbortSignal): Promise<void> {
  return new Promise((resolve) => {
    const timer = window.setTimeout(done, ms)
    function done() {
      window.clearTimeout(timer)
      signal.removeEventListener('abort', done)
      resolve()
    }
    signal.addEventListener('abort', done)
  })
}
