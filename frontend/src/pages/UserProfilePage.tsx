import { Navigate, useParams } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'
import ProfileLayout from '../components/ProfileLayout'
import type { NavItem } from '../components/ProfileLayout'
import { UserProfileOverviewPage } from './UserProfileOverviewPage'
import { UserProfilePermissionsPage } from './UserProfilePermissionsPage'
import { UserProfileChangePasswordPage } from './UserProfileChangePasswordPage'
import { UserProfileSessionsPage } from './UserProfileSessionsPage'
import { UserProfileAsQuizzerPage } from './UserProfileAsQuizzerPage'
import { UserProfileAsCoachPage } from './UserProfileAsCoachPage'
import { UserProfileAsAdminPage } from './UserProfileAsAdminPage'
import { UserProfileAsQuizmasterPage } from './UserProfileAsQuizmasterPage'
import { UserProfileAsContentJudgePage } from './UserProfileAsContentJudgePage'

type ChildRoute =
  | 'overview'
  | 'permissions'
  | 'change-password'
  | 'sessions'
  | 'as-quizzer'
  | 'as-coach'
  | 'as-admin'
  | 'as-quizmaster'
  | 'as-content-judge'

export const UserProfilePage = (props: { childRoute?: ChildRoute }) => {
  const auth = useAuth()
  const { user_id } = useParams<{ user_id: string }>()

  if (!user_id) return <Navigate to="/404" replace />
  if (auth.isCheckingAuth) return null

  const isSuperUser = auth.session?.hasRole('super_user') ?? false
  const isOwnProfile = auth.session?.userId === user_id
  const canViewPrivate = isSuperUser || isOwnProfile

  const navItems: NavItem[] = [
    { kind: 'route', label: 'Overview',             to: `/user/${user_id}/overview`         },
    { kind: 'route', label: 'As Quizzer',           to: `/user/${user_id}/as-quizzer`       },
    { kind: 'route', label: 'As Coach',             to: `/user/${user_id}/as-coach`         },
    { kind: 'route', label: 'As Admin',             to: `/user/${user_id}/as-admin`         },
    { kind: 'route', label: 'As Quizmaster',        to: `/user/${user_id}/as-quizmaster`    },
    { kind: 'route', label: 'As Content Judge',     to: `/user/${user_id}/as-content-judge` },
    ...(canViewPrivate ? [
      { kind: 'route' as const, label: 'Permissions',          to: `/user/${user_id}/permissions`      },
      { kind: 'route' as const, label: 'Change Password',      to: `/user/${user_id}/change-password`  },
      { kind: 'route' as const, label: 'Manage User Sessions', to: `/user/${user_id}/sessions`         },
    ] : []),
  ]

  return (
    <ProfileLayout title="User Profile" navItems={navItems}>
      {props.childRoute === 'overview'         && <UserProfileOverviewPage userId={user_id} />}
      {props.childRoute === 'permissions'      && (canViewPrivate ? <UserProfilePermissionsPage userId={user_id} /> : null)}
      {props.childRoute === 'change-password'  && (canViewPrivate ? <UserProfileChangePasswordPage /> : null)}
      {props.childRoute === 'sessions'         && (canViewPrivate ? <UserProfileSessionsPage /> : null)}
      {props.childRoute === 'as-quizzer'       && <UserProfileAsQuizzerPage userId={user_id} isSuperUser={isSuperUser} />}
      {props.childRoute === 'as-coach'         && <UserProfileAsCoachPage userId={user_id} isSuperUser={isSuperUser} />}
      {props.childRoute === 'as-admin'         && <UserProfileAsAdminPage userId={user_id} isSuperUser={isSuperUser} />}
      {props.childRoute === 'as-quizmaster'    && <UserProfileAsQuizmasterPage userId={user_id} isSuperUser={isSuperUser} />}
      {props.childRoute === 'as-content-judge' && <UserProfileAsContentJudgePage userId={user_id} isSuperUser={isSuperUser} />}
    </ProfileLayout>
  )
}
