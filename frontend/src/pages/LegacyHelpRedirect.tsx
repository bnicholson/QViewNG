import { Navigate, useParams } from 'react-router-dom'
import { docCategoryById, docPath, latestVersion } from '../data/docs'

/**
 * The Help Center used to live at `/help/:slug` before it was split into per-product,
 * per-version manuals. Those old links now resolve into the QView documentation.
 */
export const LegacyHelpRedirect = () => {
  const { slug } = useParams<{ slug: string }>()
  const qview = docCategoryById('qview')
  if (!qview) return <Navigate to="/help" replace />
  return <Navigate to={docPath(qview, latestVersion(qview).version, slug)} replace />
}
