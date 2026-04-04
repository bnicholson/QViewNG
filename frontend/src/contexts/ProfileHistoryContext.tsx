import { createContext, useCallback, useContext, useEffect, useRef, useState } from 'react'
import { useLocation, useNavigate } from 'react-router-dom'

function getProfileSegment(pathname: string): string | null {
  return pathname.split('/').filter(Boolean)[0] ?? null
}

interface ProfileHistoryContextValue {
  exitUrl: string | null
  exit: () => void
}

const ProfileHistoryContext = createContext<ProfileHistoryContextValue>({
  exitUrl: null,
  exit: () => {},
})

export function ProfileHistoryProvider({ children }: { children: React.ReactNode }) {
  const location = useLocation()
  const navigate = useNavigate()

  // Stack of URLs — each entry is "where to navigate when Exit is pressed from this depth".
  // stackRef[0] = exit target when in the first profile entered
  // stackRef[N] = exit target when in the (N+1)th nested profile
  const stackRef = useRef<string[]>([])

  // True when the current location change was triggered by our own exit() call.
  // Prevents the useEffect from pushing a new entry for backward navigations.
  const exitingRef = useRef(false)

  const prevUrlRef = useRef(location.pathname + location.search)
  const prevSegmentRef = useRef(getProfileSegment(location.pathname))

  const [exitUrl, setExitUrl] = useState<string | null>(null)

  useEffect(() => {
    const newSegment = getProfileSegment(location.pathname)
    const prevSegment = prevSegmentRef.current

    if (newSegment !== prevSegment) {
      if (exitingRef.current) {
        // Exit button was pressed — pop the top entry
        exitingRef.current = false
        stackRef.current = stackRef.current.slice(0, -1)
      } else {
        // Forward navigation into a new profile — push where we came from
        stackRef.current = [...stackRef.current, prevUrlRef.current]
      }
      setExitUrl(stackRef.current[stackRef.current.length - 1] ?? null)
    }

    prevUrlRef.current = location.pathname + location.search
    prevSegmentRef.current = newSegment
  }, [location.pathname, location.search])

  const exit = useCallback(() => {
    const url = stackRef.current[stackRef.current.length - 1]
    if (!url) return
    exitingRef.current = true
    navigate(url)
  }, [navigate])

  return (
    <ProfileHistoryContext.Provider value={{ exitUrl, exit }}>
      {children}
    </ProfileHistoryContext.Provider>
  )
}

export function useProfileHistory() {
  return useContext(ProfileHistoryContext)
}
