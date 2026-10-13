import { createContext, useContext, useEffect, useState } from 'react'
import type { NavItem } from '../utils/profileNav'

/** What the profile on screen exposes to navigation that lives outside it (the mobile bottom bar). */
export interface ProfileNav {
  entityType: string
  entityName?: string
  navItems: NavItem[]
}

const NavContext = createContext<ProfileNav | null>(null)
/**
 * The setter has a context of its own so a ProfileLayout that publishes its nav does not also
 * consume it — otherwise every publish would re-render the layout and publish again.
 */
const SetNavContext = createContext<(nav: ProfileNav | null) => void>(() => {})

export function ProfileNavProvider({ children }: { children: React.ReactNode }) {
  const [nav, setNav] = useState<ProfileNav | null>(null)

  return (
    <SetNavContext.Provider value={setNav}>
      <NavContext.Provider value={nav}>
        {children}
      </NavContext.Provider>
    </SetNavContext.Provider>
  )
}

/** The profile currently on screen, or null when the page is not a profile. */
export const useProfileNav = () => useContext(NavContext)

/** Publishes a profile's nav for as long as the calling component is mounted. */
export function usePublishProfileNav(nav: ProfileNav) {
  const setNav = useContext(SetNavContext)
  // Callers rebuild navItems on every render, so compare by content rather than identity.
  const signature = JSON.stringify(nav)

  useEffect(() => {
    setNav(JSON.parse(signature) as ProfileNav)
  }, [setNav, signature])

  useEffect(() => () => setNav(null), [setNav])
}
