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
import { UserProfileAsVolunteerPage } from './UserProfileAsVolunteerPage'

type ChildRoute = 'overview' | 'permissions' | 'change-password' | 'sessions' | 'as-quizzer' | 'as-coach' | 'as-volunteer'

export const UserProfilePage = (props: { childRoute?: ChildRoute }) => {
  const auth = useAuth()
  const { user_id } = useParams<{ user_id: string }>()

  if (!auth.isAuthenticated) return <Navigate to="/" replace />
  if (!user_id) return <Navigate to="/404" replace />

  const navItems: NavItem[] = [
    { kind: 'route', label: 'Overview',             to: `/user/${user_id}/overview`         },
    { kind: 'route', label: 'Permissions',          to: `/user/${user_id}/permissions`      },
    { kind: 'route', label: 'Change Password',      to: `/user/${user_id}/change-password`  },
    { kind: 'route', label: 'Manage User Sessions', to: `/user/${user_id}/sessions`         },
    { kind: 'route', label: 'As Quizzer',           to: `/user/${user_id}/as-quizzer`       },
    { kind: 'route', label: 'As Coach',             to: `/user/${user_id}/as-coach`         },
    { kind: 'route', label: 'As Volunteer',         to: `/user/${user_id}/as-volunteer`     },
  ]

  return (
    <ProfileLayout title="User Profile" navItems={navItems}>
      {props.childRoute === 'overview'        && <UserProfileOverviewPage userId={user_id} />}
      {props.childRoute === 'permissions'     && <UserProfilePermissionsPage userId={user_id} />}
      {props.childRoute === 'change-password' && <UserProfileChangePasswordPage />}
      {props.childRoute === 'sessions'        && <UserProfileSessionsPage />}
      {props.childRoute === 'as-quizzer'      && <UserProfileAsQuizzerPage />}
      {props.childRoute === 'as-coach'        && <UserProfileAsCoachPage />}
      {props.childRoute === 'as-volunteer'    && <UserProfileAsVolunteerPage />}
    </ProfileLayout>
  )
}
