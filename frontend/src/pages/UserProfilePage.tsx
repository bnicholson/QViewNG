import { useEffect, useState } from 'react'
import { PERMISSION } from '../auth/permissions'
import { Navigate, useParams } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'
import Stack from '@mui/material/Stack'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faRightFromBracket } from '@fortawesome/free-solid-svg-icons'
import ProfileLayout from '../components/ProfileLayout'
import type { NavItem } from '../components/ProfileLayout'
import { ProfileBreadcrumbs } from '../components/ProfileBreadcrumbs'
import { UserProfileOverviewPage } from './UserProfileOverviewPage'
import { UserProfilePermissionsPage } from './UserProfilePermissionsPage'
import { UserProfileChangePasswordPage } from './UserProfileChangePasswordPage'
import { UserProfileSessionsPage } from './UserProfileSessionsPage'
import { UserProfileMyTeamsPage } from './UserProfileMyTeamsPage'
import { UserProfileAsCoachQuizzerRostersPage } from './UserProfileAsCoachQuizzerRostersPage'
import { UserProfileAsCoachGearPage } from './UserProfileAsCoachGearPage'
import { UserProfileAsAdminPage } from './UserProfileAsAdminPage'
import { UserProfileAsQuizmasterPage } from './UserProfileAsQuizmasterPage'
import { UserProfileAsContentJudgePage } from './UserProfileAsContentJudgePage'
import { UserProfileManagedTournamentsPage } from './UserProfileManagedTournamentsPage'
import { UserProfileManagedTournamentGroupsPage } from './UserProfileManagedTournamentGroupsPage'

type ChildRoute =
  | 'overview'
  | 'permissions'
  | 'change-password'
  | 'sessions'
  | 'teams'
  | 'my-rosters'
  | 'my-gear'
  | 'as-admin'
  | 'as-quizmaster'
  | 'as-content-judge'
  | 'managed-tournaments'
  | 'managed-tournament-groups'

export const UserProfilePage = (props: { childRoute?: ChildRoute }) => {
  const auth = useAuth()
  const { user_id } = useParams<{ user_id: string }>()
  const [userName, setUserName] = useState<{ fname: string; lname: string } | null>(null)
  const [targetCanAdminister, setTargetCanAdminister] = useState<boolean | null>(null)
  const [targetCanCreateTournaments, setTargetCanCreateTournaments] = useState<boolean>(false)
  const [targetIsDeactivated, setTargetIsDeactivated] = useState<boolean>(false)

  useEffect(() => {
    if (!user_id) return
    setTargetCanAdminister(null)
    setTargetIsDeactivated(false)
    Promise.all([
      fetch(`/api/users/${user_id}`).then((r) => r.ok ? r.json() : null),
      fetch(`/api/users/${user_id}/roles-and-permissions`).then((r) => r.ok ? r.json() : null),
    ]).then(([userData, rolesData]) => {
      if (userData) {
        setUserName({ fname: userData.fname, lname: userData.lname })
        setTargetIsDeactivated(userData.del_fl === true)
      }
      // What the target user may do, not which role says so.
      setTargetCanAdminister(rolesData?.permissions?.includes(PERMISSION.SYSTEM_ADMINISTER) ?? false)
      setTargetCanCreateTournaments(rolesData?.permissions?.includes(PERMISSION.TOURNAMENT_CREATE) ?? false)
    }).catch(() => { setTargetCanAdminister(false) })
  }, [user_id])

  if (!user_id) return <Navigate to="/404" replace />
  if (auth.isCheckingAuth || targetCanAdminister === null) return null

  const canAdminister = auth.session?.isSystemAdministrator ?? false
  const canReadAnyUser = auth.session?.can(PERMISSION.USER_READ_ANY) ?? false
  const canViewPairingCode = auth.session?.can(PERMISSION.TOURNAMENT_READ_PAIRING_CODE) ?? false

  // An unrestricted account's profile is only reachable by another unrestricted account.
  if (targetCanAdminister && !canAdminister) return <Navigate to="/404" replace />

  // A deactivated (soft-deleted) user's profile is not viewable. Their name still resolves
  // elsewhere (data tables, references), but navigating here shows a deactivation notice.
  if (targetIsDeactivated) {
    return (
      <div style={{ maxWidth: 640, margin: '4rem auto', padding: '2rem', textAlign: 'center' }}>
        <h1 style={{ marginBottom: '0.5rem' }}>
          {userName ? `${userName.fname} ${userName.lname}` : 'This user'}
        </h1>
        <p style={{ fontSize: '1.1rem', color: '#666' }}>
          This user has been deactivated. Their profile is no longer available.
        </p>
      </div>
    )
  }

  const isOwnProfile = auth.session?.userId === user_id
  // Managing tournaments is the same capability as being able to create one.
  const isTournamentManager = auth.session?.can(PERMISSION.TOURNAMENT_CREATE) ?? false
  const canViewPrivate = canReadAnyUser || isOwnProfile
  const canManageTournaments = isOwnProfile || canReadAnyUser
  const canCreateTournament = auth.session?.can(PERMISSION.TOURNAMENT_CREATE) ?? false
  const canDeleteTournament = auth.session?.can(PERMISSION.TOURNAMENT_DELETE) ?? false
  const canCreateTournamentGroup = auth.session?.can(PERMISSION.TOURNAMENTGROUP_CREATE) ?? false
  const canDeleteTournamentGroup = auth.session?.can(PERMISSION.TOURNAMENTGROUP_DELETE) ?? false
  const canManageTournamentGroups = (isOwnProfile || canReadAnyUser) && isTournamentManager && canCreateTournamentGroup

  const navItems: NavItem[] = [
    { kind: 'route', label: 'Overview',         to: `/user/${user_id}/overview`     },
    { kind: 'route', label: 'My Teams',         to: `/user/${user_id}/teams`     },
    ...(canViewPrivate ? [
      { kind: 'route' as const, label: 'My Rosters', to: `/user/${user_id}/my-rosters` },
      { kind: 'route' as const, label: 'My Gear',    to: `/user/${user_id}/my-gear`    },
    ] : []),
    { kind: 'route', label: 'As Quizmaster',        to: `/user/${user_id}/as-quizmaster`    },
    { kind: 'route', label: 'As Content Judge',     to: `/user/${user_id}/as-content-judge` },
    { kind: 'route', label: 'As Admin',             to: `/user/${user_id}/as-admin`         },
    ...(canManageTournaments ? [
      { kind: 'route' as const, label: 'Managed Tournaments',       to: `/user/${user_id}/managed-tournaments`        },
    ] : []),
    ...(canManageTournamentGroups ? [
      { kind: 'route' as const, label: 'Managed Tournament Groups', to: `/user/${user_id}/managed-tournament-groups`  },
    ] : []),
    ...(canViewPrivate ? [
      { kind: 'route' as const, label: 'Permissions',          to: `/user/${user_id}/permissions`      },
      { kind: 'route' as const, label: 'Change Password',      to: `/user/${user_id}/change-password`  },
      { kind: 'route' as const, label: 'Manage User Sessions', to: `/user/${user_id}/sessions`         },
    ] : []),
    // Dev tools link (moved here from the global nav bar).
    ...((auth.session?.can(PERMISSION.SYSTEM_ACCESS_DEV_TOOLS) ?? false) ? [
      { kind: 'route' as const, label: 'Developer', to: `/dev` },
    ] : []),
  ]

  return (
    <ProfileLayout entityType="User" entityName={userName ? `${userName.fname} ${userName.lname}` : ''} navItems={navItems}>
      <Stack spacing={3}>

        {/* ── Breadcrumb (shown on every page of the profile) ── */}
        <ProfileBreadcrumbs
          crumbs={[
            { name: 'Home', to: '/' },
            { label: 'User', name: userName ? `${userName.fname} ${userName.lname}` : '' },
          ]}
          actions={isOwnProfile && (
            // Mobile only: there the top bar drops its Logout icon, so your own profile carries it.
            <Button
              size="small"
              variant="outlined"
              color="inherit"
              onClick={() => auth.logout()}
              startIcon={<FontAwesomeIcon icon={faRightFromBracket} style={{ fontSize: '0.9rem' }} />}
              sx={{ display: { xs: 'inline-flex', md: 'none' }, color: 'text.secondary', borderColor: 'divider', whiteSpace: 'nowrap' }}
            >
              Logout
            </Button>
          )}
        />

        {/* ── Section content ── */}
        <Box sx={{ overflowX: 'auto' }}>
          {props.childRoute === 'overview'         && <UserProfileOverviewPage userId={user_id} />}
          {props.childRoute === 'permissions'      && (canViewPrivate ? <UserProfilePermissionsPage userId={user_id} /> : null)}
          {props.childRoute === 'change-password'  && (canViewPrivate ? <UserProfileChangePasswordPage /> : null)}
          {props.childRoute === 'sessions'         && (canViewPrivate ? <UserProfileSessionsPage /> : null)}
          {props.childRoute === 'teams'   && <UserProfileMyTeamsPage userId={user_id} canAdminister={canAdminister} isOwnProfile={isOwnProfile} />}
          {props.childRoute === 'my-rosters' && <UserProfileAsCoachQuizzerRostersPage userId={user_id} canAdminister={canAdminister} />}
          {props.childRoute === 'my-gear'             && <UserProfileAsCoachGearPage userId={user_id} canAdminister={canAdminister} isOwnProfile={isOwnProfile} />}
          {props.childRoute === 'as-admin'         && <UserProfileAsAdminPage userId={user_id} canAdminister={canAdminister} isOwnProfile={isOwnProfile} />}
          {props.childRoute === 'as-quizmaster'    && <UserProfileAsQuizmasterPage userId={user_id} canAdminister={canAdminister} isOwnProfile={isOwnProfile} />}
          {props.childRoute === 'as-content-judge'         && <UserProfileAsContentJudgePage userId={user_id} canAdminister={canAdminister} isOwnProfile={isOwnProfile} />}
          {props.childRoute === 'managed-tournaments'       && canManageTournaments       && <UserProfileManagedTournamentsPage userId={user_id} canCreate={canCreateTournament} canDelete={canDeleteTournament} isTournamentManager={isTournamentManager} canViewPairingCode={canViewPairingCode} canAdminister={canAdminister} isOwnProfile={isOwnProfile} targetCanAdminister={targetCanAdminister ?? false} targetCanCreateTournaments={targetCanCreateTournaments} />}
          {props.childRoute === 'managed-tournament-groups' && canManageTournamentGroups  && <UserProfileManagedTournamentGroupsPage userId={user_id} canCreate={canCreateTournamentGroup} canDelete={canDeleteTournamentGroup} canAdminister={canAdminister} isOwnProfile={isOwnProfile} targetCanCreateTournaments={targetCanCreateTournaments} />}
        </Box>

      </Stack>
    </ProfileLayout>
  )
}
