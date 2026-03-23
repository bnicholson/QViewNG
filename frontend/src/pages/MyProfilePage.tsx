import { Navigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'
import ProfileLayout from '../components/ProfileLayout'
import type { NavItem } from '../components/ProfileLayout'
import { MyProfileOverviewPage } from './MyProfileOverviewPage'
import { MyProfilePermissionsPage } from './MyProfilePermissionsPage'
import { MyProfileChangePasswordPage } from './MyProfileChangePasswordPage'
import { MyProfileSessionsPage } from './MyProfileSessionsPage'

type ChildRoute = 'overview' | 'permissions' | 'change-password' | 'sessions'

const NAV_ITEMS: NavItem[] = [
  { kind: 'route', label: 'Overview',              to: '/my-profile/overview'         },
  { kind: 'route', label: 'Permissions',           to: '/my-profile/permissions'      },
  { kind: 'route', label: 'Change Password',       to: '/my-profile/change-password'  },
  { kind: 'route', label: 'Manage User Sessions',  to: '/my-profile/sessions'         },
]

export const MyProfilePage = (props: { childRoute?: ChildRoute }) => {
  const auth = useAuth()
  if (!auth.isAuthenticated) return <Navigate to="/" replace />

  return (
    <ProfileLayout title="My Profile" navItems={NAV_ITEMS}>
      {props.childRoute === 'overview'         && <MyProfileOverviewPage />}
      {props.childRoute === 'permissions'      && <MyProfilePermissionsPage />}
      {props.childRoute === 'change-password'  && <MyProfileChangePasswordPage />}
      {props.childRoute === 'sessions'         && <MyProfileSessionsPage />}
    </ProfileLayout>
  )
}
