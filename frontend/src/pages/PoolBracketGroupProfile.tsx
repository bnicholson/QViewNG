import { useState, useEffect, useCallback } from 'react'
import { PERMISSION } from '../auth/permissions'
import { Navigate, useParams } from 'react-router'
import Box from '@mui/material/Box'
import Stack from '@mui/material/Stack'
import ProfileLayout from '../components/ProfileLayout'
import { ProfileBreadcrumbs } from '../components/ProfileBreadcrumbs'
import { PoolBracketGroupAPI, type PoolBracketGroupTS } from '../features/PoolBracketGroupAPI'
import { DivisionAPI, type DivisionTS } from '../features/DivisionAPI'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import { useTournamentAccess } from '../hooks/useTournamentAccess'
import PoolBracketsTable from '../components/PoolBracketsTable'
import { PoolBracketGroupEditorDialog } from '../components/PoolBracketGroupEditorDialog'
import { PoolBracketGroupProfileOverviewPage } from './PoolBracketGroupProfileOverviewPage'

export const PoolBracketGroupProfile = (props: { childRoute?: string }) => {
  const { groupid } = useParams()
  if (!groupid) return <></>

  const [group, setGroup] = useState<PoolBracketGroupTS | null>(null)
  const [division, setDivision] = useState<DivisionTS | null>(null)
  const [tournament, setTournament] = useState<TournamentTS | null>(null)
  const [notFound, setNotFound] = useState(false)
  const [editorIsOpen, setEditorIsOpen] = useState(false)
  // Pool / bracket counts shown on the Overview.
  const [counts, setCounts] = useState({ pools: 0, brackets: 0 })

  useEffect(() => {
    let cancelled = false
    PoolBracketGroupAPI.getById(groupid)
      .then(async grp => {
        if (cancelled) return
        setGroup(grp)
        const div = await DivisionAPI.getById(grp.divisionid)
        if (cancelled) return
        setDivision(div)
        const tour = await TournamentAPI.getById(div.tid)
        if (cancelled) return
        setTournament(tour)
      })
      .catch(() => { if (!cancelled) setNotFound(true) })
    return () => { cancelled = true }
  }, [groupid])

  const loadCounts = useCallback(() => {
    PoolBracketGroupAPI.getPoolBrackets(groupid)
      .then(items => setCounts({
        pools: items.filter(b => b.type === 'pool').length,
        brackets: items.filter(b => b.type === 'bracket').length,
      }))
      .catch(() => { /* leave the counts as they are if they can't be loaded */ })
  }, [groupid])

  // Reload whenever the Overview is (re)entered, so counts reflect pools/brackets added on the
  // other tabs — this component stays mounted while the child route changes.
  useEffect(() => {
    if (props.childRoute === 'overview') loadCounts()
  }, [props.childRoute, loadCounts])

  const access = useTournamentAccess(tournament?.tid, tournament?.owner_id)

  if (notFound) return <Navigate to="/404" replace />
  if (!group || !division || !tournament) return <div>Loading Pool/Bracket Group…</div>

  const { canViewAuditColumns, canCreate, canRead } = access

  const navItems = [
    ...(canRead(PERMISSION.POOLBRACKETGROUP_READ) ? [{ kind: 'route' as const, label: 'Overview', to: `/pool-bracket-group/${groupid}/overview` }] : []),
    ...(canRead(PERMISSION.POOLBRACKET_READ)      ? [{ kind: 'route' as const, label: 'Pools',    to: `/pool-bracket-group/${groupid}/pools`    }] : []),
    ...(canRead(PERMISSION.POOLBRACKET_READ)      ? [{ kind: 'route' as const, label: 'Brackets', to: `/pool-bracket-group/${groupid}/brackets` }] : []),
  ]

  return (
    <ProfileLayout title={<>Pool/Bracket Group:<br />{group.name}</>} navItems={navItems}>
      <Stack spacing={3}>

        <ProfileBreadcrumbs crumbs={[
          { name: 'Home', to: '/' },
          { label: 'Tournament', name: tournament.tname, to: `/tournament/${tournament.tid}/overview` },
          { label: 'Division', name: division.dname, to: `/division/${division.did}/overview` },
          { label: 'Pool/Bracket Group', name: group.name },
        ]} />

        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview' && (
            <PoolBracketGroupProfileOverviewPage
              group={group}
              division={division}
              poolCount={counts.pools}
              bracketCount={counts.brackets}
              onEdit={() => setEditorIsOpen(true)}
              canEdit={canCreate(PERMISSION.POOLBRACKETGROUP_UPDATE)}
            />
          )}
          {props.childRoute === 'pools' && (
            // Pools in this group. Division and Group are fixed context here, so anything created
            // joins this group.
            <PoolBracketsTable tid={tournament.tid} did={division.did} poolBracketGroupId={groupid}
              type="pool" entityLabel="Pool" title="Pools"
              showCreateButton={canCreate(PERMISSION.POOLBRACKET_CREATE)}
              showEditButton={canCreate(PERMISSION.POOLBRACKET_UPDATE)}
              showDeleteButton={canCreate(PERMISSION.POOLBRACKET_DELETE)}
              showAuditColumns={canViewAuditColumns}
              hiddenColumns={['Division']} />
          )}
          {props.childRoute === 'brackets' && (
            <PoolBracketsTable tid={tournament.tid} did={division.did} poolBracketGroupId={groupid}
              type="bracket" entityLabel="Bracket" title="Brackets"
              showCreateButton={canCreate(PERMISSION.POOLBRACKET_CREATE)}
              showEditButton={canCreate(PERMISSION.POOLBRACKET_UPDATE)}
              showDeleteButton={canCreate(PERMISSION.POOLBRACKET_DELETE)}
              showAuditColumns={canViewAuditColumns}
              hiddenColumns={['Division']} />
          )}
        </Box>

      </Stack>

      <PoolBracketGroupEditorDialog
        tid={tournament.tid}
        lockedDivisionId={division.did}
        group={group}
        isOpen={editorIsOpen}
        onCancel={() => setEditorIsOpen(false)}
        onSave={(updated) => { setEditorIsOpen(false); setGroup(updated) }}
      />
    </ProfileLayout>
  )
}
