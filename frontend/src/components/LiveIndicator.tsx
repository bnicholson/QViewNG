import { useTheme } from '@mui/material/styles'
import type { LiveStatus } from '../hooks/useLiveFeed'

/**
 * A small dot and label saying whether a page is receiving live updates (see useLiveFeed).
 * Shows nothing for `error`, which the page reports itself.
 */
export function LiveIndicator({ status, pollIntervalMs }: { status: LiveStatus; pollIntervalMs: number }) {
  const theme = useTheme()
  if (status === 'error') return null

  const { color, label, title } = {
    live: {
      color: theme.palette.success.main,
      label: 'Live',
      title: 'Updates appear as soon as the server receives them.',
    },
    connecting: {
      color: theme.palette.grey[500],
      label: 'Connecting…',
      title: 'Opening the live connection.',
    },
    reconnecting: {
      color: theme.palette.warning.main,
      label: 'Reconnecting…',
      title: 'The live connection dropped and is being retried.',
    },
    polling: {
      color: theme.palette.warning.main,
      label: `Refreshing every ${Math.round(pollIntervalMs / 1000)}s`,
      title: 'The live connection is unavailable, so the page is refreshing on a timer and will switch back when it can.',
    },
  }[status]

  return (
    <span
      title={title}
      style={{ display: 'inline-flex', alignItems: 'center', gap: 6, fontSize: 12, color: theme.palette.text.secondary }}
    >
      <span
        aria-hidden
        style={{ width: 8, height: 8, borderRadius: '50%', background: color, display: 'inline-block' }}
      />
      {label}
    </span>
  )
}
