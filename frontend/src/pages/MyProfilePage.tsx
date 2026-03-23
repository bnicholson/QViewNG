import ProfileLayout from '../components/ProfileLayout'
import type { NavItem } from '../components/ProfileLayout'
import { MyProfileOverviewPage } from './MyProfileOverviewPage'
import { MyProfilePermissionsPage } from './MyProfilePermissionsPage'

type ChildRoute = 'overview' | 'permissions'

const NAV_ITEMS: NavItem[] = [
  { kind: 'route', label: 'Overview',    to: '/my-profile/overview'    },
  { kind: 'route', label: 'Permissions', to: '/my-profile/permissions' },
]

export const MyProfilePage = (props: { childRoute?: ChildRoute }) => {
  return (
    <ProfileLayout title="My Profile" navItems={NAV_ITEMS}>
      {props.childRoute === 'overview'    && <MyProfileOverviewPage />}
      {props.childRoute === 'permissions' && <MyProfilePermissionsPage />}
    </ProfileLayout>
  )
}
