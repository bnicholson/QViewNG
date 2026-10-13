import { createContext, useContext, useEffect, useState } from 'react'
import type { NavItem } from '../utils/profileNav'
import type { DocCategoryId } from '../data/docs'

/**
 * What the page on screen offers in the mobile bottom bar's Menu sheet. Plain data, so a publish
 * can be compared by content (see `usePublishMobileMenu`).
 */
export type MobileMenu =
  | {
      kind: 'profile'
      entityType: string
      entityName?: string
      navItems: NavItem[]
    }
  | {
      kind: 'docs'
      /** The manual on screen; the sheet opens on it. */
      categoryId: DocCategoryId
      /** As it appears in the URL; absent on the Help Center landing page. */
      version?: string
      activeSlug?: string
    }

const MenuContext = createContext<MobileMenu | null>(null)
/**
 * The setter has a context of its own so a page that publishes its menu does not also consume
 * it — otherwise every publish would re-render the page and publish again.
 */
const SetMenuContext = createContext<(menu: MobileMenu | null) => void>(() => {})

export function MobileMenuProvider({ children }: { children: React.ReactNode }) {
  const [menu, setMenu] = useState<MobileMenu | null>(null)

  return (
    <SetMenuContext.Provider value={setMenu}>
      <MenuContext.Provider value={menu}>
        {children}
      </MenuContext.Provider>
    </SetMenuContext.Provider>
  )
}

/** The menu the page on screen offers, or null when it offers none. */
export const useMobileMenu = () => useContext(MenuContext)

/** Publishes a menu for as long as the calling component is mounted. */
export function usePublishMobileMenu(menu: MobileMenu) {
  const setMenu = useContext(SetMenuContext)
  // Callers rebuild their menu on every render, so compare by content rather than identity.
  const signature = JSON.stringify(menu)

  useEffect(() => {
    setMenu(JSON.parse(signature) as MobileMenu)
  }, [setMenu, signature])

  useEffect(() => () => setMenu(null), [setMenu])
}
