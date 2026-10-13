/** A link in a profile's section nav: another route, or a section further down the same page. */
export type RouteNavItem = {
  kind: 'route'
  label: string
  to: string
  /** Offered only in the mobile Menu sheet, not the desktop sidebar. */
  mobileOnly?: boolean
}

export type AnchorNavItem = {
  kind: 'anchor'
  label: string
  anchor: string
  /** Offered only in the mobile Menu sheet, not the desktop sidebar. */
  mobileOnly?: boolean
}

export type NavItem = RouteNavItem | AnchorNavItem

export function isRouteNavItem(item: NavItem): item is RouteNavItem {
  return item.kind === 'route'
}

export function isRouteActive(pathname: string, to: string) {
  return pathname === to || pathname.startsWith(to + '/')
}
