import { useEffect, useRef, useState } from 'react'
import Box from '@mui/material/Box'
import Divider from '@mui/material/Divider'
import FormControl from '@mui/material/FormControl'
import InputLabel from '@mui/material/InputLabel'
import MenuItem from '@mui/material/MenuItem'
import Select from '@mui/material/Select'
import ToggleButton from '@mui/material/ToggleButton'
import ToggleButtonGroup from '@mui/material/ToggleButtonGroup'
import Typography from '@mui/material/Typography'
import {
  docCategories,
  docCategoryById,
  docPath,
  latestVersion,
  resolveVersion,
  type DocCategoryId,
} from '../data/docs'
import type { MobileMenu } from '../contexts/MobileMenuContext'
import DocsNavTree from './DocsNavTree'
import MenuSheetHeader from './MenuSheetHeader'

interface DocsMenuPanelProps {
  menu: Extract<MobileMenu, { kind: 'docs' }>
  onClose: () => void
}

/**
 * The Help Center's navigation in the mobile Menu sheet: a switcher for the three manuals, a
 * version picker, and the chosen manual's article tree.
 *
 * Unlike the desktop sidebar, switching manual or version here only changes what the sheet shows —
 * navigating would close the sheet before an article could be picked. Tapping an article navigates.
 */
export default function DocsMenuPanel({ menu, onClose }: DocsMenuPanelProps) {
  const current = docCategoryById(menu.categoryId) ?? docCategories[0]
  const currentVersion = resolveVersion(current, menu.version)

  const [browsedId, setBrowsedId] = useState<DocCategoryId>(current.id)
  const [browsedVersion, setBrowsedVersion] = useState(currentVersion.version)
  const browsed = docCategoryById(browsedId) ?? current
  const docVersion = resolveVersion(browsed, browsedVersion)

  // The page's article is only highlighted while its own manual and version are on show.
  const showingCurrent = browsed.id === current.id && docVersion.version === currentVersion.version
  const activeSlug = showingCurrent ? menu.activeSlug : undefined

  const onManualChange = (_: React.MouseEvent, id: DocCategoryId | null) => {
    const next = docCategoryById(id ?? undefined)
    if (!next) return
    setBrowsedId(next.id)
    // Returning to the page's own manual returns to its version too.
    setBrowsedVersion(next.id === current.id ? currentVersion.version : latestVersion(next).version)
  }

  // Open with the page's article in view rather than at the top of a long tree. Scrolled by hand:
  // scrollIntoView would also try to scroll the (locked) page behind the sheet.
  const treeRef = useRef<HTMLDivElement>(null)
  useEffect(() => {
    const box = treeRef.current
    const active = box?.querySelector<HTMLElement>('[aria-current="page"]')
    if (box && active) box.scrollTop = active.offsetTop - box.clientHeight / 2
  }, [])

  return (
    <>
      <MenuSheetHeader
        overline="Help Center"
        title={<>{browsed.title} <Typography component="span" color="text.secondary">v{docVersion.version}</Typography></>}
        onClose={onClose}
      />
      <Box sx={{ px: 2, pb: 1.5, display: 'flex', flexDirection: 'column', gap: 1.5 }}>
        <ToggleButtonGroup
          value={browsed.id}
          exclusive
          fullWidth
          size="small"
          color="primary"
          onChange={onManualChange}
          aria-label="Manual"
        >
          {docCategories.map((c) => (
            <ToggleButton key={c.id} value={c.id} sx={{ px: 0.5 }}>
              {c.title}
            </ToggleButton>
          ))}
        </ToggleButtonGroup>

        {browsed.versions.length > 1 && (
          <FormControl size="small" fullWidth>
            <InputLabel id="docs-menu-version-label">Version</InputLabel>
            <Select
              labelId="docs-menu-version-label"
              label="Version"
              value={docVersion.version}
              onChange={(e) => setBrowsedVersion(e.target.value)}
            >
              {browsed.versions.map((v) => (
                <MenuItem key={v.version} value={v.version}>
                  {v.version}
                  {v.released ? ` · ${v.released}` : ''}
                </MenuItem>
              ))}
            </Select>
          </FormControl>
        )}
      </Box>
      <Divider />
      <Box ref={treeRef} sx={{ position: 'relative', flex: 1, minHeight: 0, overflowY: 'auto', px: 1, pb: 2 }}>
        {/* Keyed so each manual/version starts with its own folders open, not the last one's. */}
        <DocsNavTree
          key={`${browsed.id}/${docVersion.version}`}
          nav={docVersion.nav}
          basePath={docPath(browsed, docVersion.version)}
          activeSlug={activeSlug}
          onSelect={onClose}
          touch
        />
      </Box>
    </>
  )
}
