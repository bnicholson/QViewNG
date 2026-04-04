import { createContext, useContext, useEffect, useRef, useState } from 'react'
import { useLocation } from 'react-router-dom'

function getProfileSegment(pathname: string): string | null {
  const segment = pathname.split('/').filter(Boolean)[0] ?? null
  return segment ?? null
}

interface ProfileHistoryContextValue {
  exitUrl: string | null
}

const ProfileHistoryContext = createContext<ProfileHistoryContextValue>({ exitUrl: null })

export function ProfileHistoryProvider({ children }: { children: React.ReactNode }) {
  const location = useLocation()
  const currentUrl = location.pathname + location.search

  const prevUrlRef = useRef<string>(currentUrl)
  const prevSegmentRef = useRef<string | null>(getProfileSegment(location.pathname))
  const [exitUrl, setExitUrl] = useState<string | null>(null)

  useEffect(() => {
    const newSegment = getProfileSegment(location.pathname)
    const prevSegment = prevSegmentRef.current

    if (newSegment !== prevSegment) {
      // Entered a different profile (or left all profiles) — record where we came from
      setExitUrl(prevUrlRef.current)
    }

    prevUrlRef.current = location.pathname + location.search
    prevSegmentRef.current = newSegment
  }, [location.pathname, location.search])

  return (
    <ProfileHistoryContext.Provider value={{ exitUrl }}>
      {children}
    </ProfileHistoryContext.Provider>
  )
}

export function useProfileHistory() {
  return useContext(ProfileHistoryContext)
}
