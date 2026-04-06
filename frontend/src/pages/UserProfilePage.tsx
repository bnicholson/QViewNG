import { useEffect, useState } from 'react'
import { Navigate, useParams } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'
import ProfileLayout from '../components/ProfileLayout'
import type { NavItem } from '../components/ProfileLayout'
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
  const [targetIsSuperUser, setTargetIsSuperUser] = useState<boolean | null>(null)

  useEffect(() => {
    if (!user_id) return
    setTargetIsSuperUser(null)
    Promise.all([
      fetch(`/api/users/${user_id}`).then((r) => r.ok ? r.json() : null),
      fetch(`/api/users/${user_id}/roles-and-permissions`).then((r) => r.ok ? r.json() : null),
    ]).then(([userData, rolesData]) => {
      if (userData) setUserName({ fname: userData.fname, lname: userData.lname })
      setTargetIsSuperUser(rolesData?.roles?.includes('super_user') ?? false)
    }).catch(() => { setTargetIsSuperUser(false) })
  }, [user_id])

  if (!user_id) return <Navigate to="/404" replace />
  if (auth.isCheckingAuth || targetIsSuperUser === null) return null

  const isSuperUser = auth.session?.hasRole('super_user') ?? false

  if (targetIsSuperUser && !isSuperUser) return <Navigate to="/404" replace />

  const isOwnProfile = auth.session?.userId === user_id
  const canViewPrivate = isSuperUser || isOwnProfile
  const canManageTournaments = (isOwnProfile || isSuperUser) && (isSuperUser || (auth.session?.hasPermission('tournament:create') ?? false))
  const canManageTournamentGroups = (isOwnProfile || isSuperUser) && (isSuperUser || (auth.session?.hasPermission('tournamentgroup:create') ?? false))
  const canCreateTournament = isSuperUser || (auth.session?.hasPermission('tournament:create') ?? false)
  const canDeleteTournament = isSuperUser || (auth.session?.hasPermission('tournament:delete') ?? false)
  const canCreateTournamentGroup = isSuperUser || (auth.session?.hasPermission('tournamentgroup:create') ?? false)
  const canDeleteTournamentGroup = isSuperUser || (auth.session?.hasPermission('tournamentgroup:delete') ?? false)

  const navItems: NavItem[] = [
    { kind: 'route', label: 'Overview',         to: `/user/${user_id}/overview`     },
    { kind: 'route', label: 'Teams',         to: `/user/${user_id}/teams`     },
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
  ]

  return (
    <ProfileLayout title={<>User:<br />{userName ? `${userName.fname} ${userName.lname}` : ''}</>} navItems={navItems}>
      {props.childRoute === 'overview'         && <UserProfileOverviewPage userId={user_id} />}
      {props.childRoute === 'permissions'      && (canViewPrivate ? <UserProfilePermissionsPage userId={user_id} /> : null)}
      {props.childRoute === 'change-password'  && (canViewPrivate ? <UserProfileChangePasswordPage /> : null)}
      {props.childRoute === 'sessions'         && (canViewPrivate ? <UserProfileSessionsPage /> : null)}
      {props.childRoute === 'teams'   && <UserProfileMyTeamsPage userId={user_id} isSuperUser={isSuperUser} />}
      {props.childRoute === 'my-rosters' && <UserProfileAsCoachQuizzerRostersPage userId={user_id} isSuperUser={isSuperUser} />}
      {props.childRoute === 'my-gear'             && <UserProfileAsCoachGearPage userId={user_id} isSuperUser={isSuperUser} />}
      {props.childRoute === 'as-admin'         && <UserProfileAsAdminPage userId={user_id} isSuperUser={isSuperUser} />}
      {props.childRoute === 'as-quizmaster'    && <UserProfileAsQuizmasterPage userId={user_id} isSuperUser={isSuperUser} />}
      {props.childRoute === 'as-content-judge'         && <UserProfileAsContentJudgePage userId={user_id} isSuperUser={isSuperUser} />}
      {props.childRoute === 'managed-tournaments'       && canManageTournaments       && <UserProfileManagedTournamentsPage userId={user_id} canCreate={canCreateTournament} canDelete={canDeleteTournament} />}
      {props.childRoute === 'managed-tournament-groups' && canManageTournamentGroups  && <UserProfileManagedTournamentGroupsPage userId={user_id} canCreate={canCreateTournamentGroup} canDelete={canDeleteTournamentGroup} />}
    </ProfileLayout>
  )
}
