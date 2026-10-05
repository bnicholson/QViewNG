import { useEffect } from 'react'
import { useLocation } from 'react-router-dom'
import { titleForPath } from '../pageTitles'

/**
 * Keeps `document.title` in step with the route. Mounted once, in `App`, so every page gets a
 * title without having to set one itself — see `pageTitles.ts` for the route-to-title table.
 */
export function usePageTitle() {
  const { pathname } = useLocation()

  useEffect(() => {
    document.title = titleForPath(pathname)
  }, [pathname])
}
