import React, { createContext, useCallback, useContext, useEffect, useState } from 'react'
import { useLocation } from 'react-router-dom'
import { PERMISSION } from '../auth/permissions'

type ID = string

interface AccessTokenClaims {
  sub: string
  exp: number
  roles: string[]
  permissions: string[]
}

export interface UserSessionResponse {
  sessions: Array<{
    id: number
    device: string | null
    created_at: string
    refresh_token: string
    user_id: string
  }>
  num_pages: number
}

const MILLISECONDS_UNTIL_EXPIRY_CHECK = 10 * 1000 // check expiry every 10 seconds
const REMAINING_TOKEN_EXPIRY_TIME_ALLOWED = 60 * 1000 // 1 minute before token should be refreshed

/**
 * The permissions carried by the access token. Deliberately exposes no role accessor: every
 * access decision in the UI resolves against a permission, so the token's `roles` claim is
 * informational only (a user's roles are shown on their Permissions page, read from the API).
 */
class Permissions {
  private readonly permissionsSet: Set<string>
  private readonly permissionsArray: string[]

  constructor(perms: string[]) {
    this.permissionsArray = perms
    this.permissionsSet = new Set(this.permissionsArray)
  }

  public get permissions(): string[] {
    return this.permissionsArray
  }

  /** Exact check — true only when this permission was granted. */
  public hasPermission = (permission: string): boolean => {
    return this.permissionsSet.has(permission)
  }

  /**
   * The check to reach for. True when the permission was granted, or when the user holds
   * `system:administer` — mirroring the backend's single unrestricted-access bypass in
   * `is_rbac_and_abac_authorized`, so the UI and the API agree on who may act.
   */
  public can = (permission: string): boolean => {
    return this.permissionsSet.has(permission)
      || this.permissionsSet.has(PERMISSION.SYSTEM_ADMINISTER)
  }

  public get isSystemAdministrator(): boolean {
    return this.permissionsSet.has(PERMISSION.SYSTEM_ADMINISTER)
  }
}

interface Session {
  expiresOnUTC: number
  userId: ID
  permissions: string[]
  hasPermission(permission: string): boolean
  can(permission: string): boolean
  isSystemAdministrator: boolean
}

interface AuthContext {
  accessToken: string | undefined
  session: Session | undefined
  setAccessToken: (accessToken: string | undefined) => void
  setSession: (session: Session | undefined) => void

  isCheckingAuth: boolean
  setCheckingAuth: (checking: boolean) => void
}

interface AuthWrapperProps {
  children: React.ReactNode
}

const Context = createContext<AuthContext>(undefined as any)

// Seeds the in-memory access token (and session) from the refresh cookie. Requires a prior
// login, tracked by the `qview_session` flag. Returns true when a token is now present. This
// is the single place the token gets minted from a refresh, shared by the periodic auth check
// and by callers that need to guarantee a token before hitting an auth-gated endpoint.
async function seedAccessToken(context: AuthContext): Promise<boolean> {
  if (!localStorage.getItem('qview_session')) return false
  try {
    const response = await fetch('/api/auth/refresh', { method: 'POST' })
    if (!response.ok) {
      context.setAccessToken(undefined)
      context.setSession(undefined)
      localStorage.removeItem('qview_session')
      return false
    }
    const responseJson = await response.json()
    const parsedToken = parseJwt(responseJson.access_token) as AccessTokenClaims
    const permissions = new Permissions(parsedToken.permissions)
    context.setAccessToken(responseJson.access_token)
    context.setSession({
      userId: parsedToken.sub,
      expiresOnUTC: parsedToken.exp,
      permissions: permissions.permissions,
      hasPermission: permissions.hasPermission,
      can: permissions.can,
      isSystemAdministrator: permissions.isSystemAdministrator,
    })
    return true
  } catch {
    return false
  }
}

export const AuthProvider = (props: AuthWrapperProps) => {
  const [accessToken, setAccessToken] = useState<string | undefined>()
  const [session, setSession] = useState<Session | undefined>()
  const [isCheckingAuth, setCheckingAuth] = useState<boolean>(true)

  return (
      <Context.Provider
          value={{
            accessToken,
            session,
            setAccessToken,
            setSession,
            isCheckingAuth,
            setCheckingAuth,
          }}
      >
        {props.children}
      </Context.Provider>
  )
}

export const useAuth = () => {
  const context = useContext(Context)

  const login = async (email: string, password: string): Promise<string | null> => {
    
    let requestBody = {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        identifier: email,
        password,
        screen_width: window.screen.width,
        screen_height: window.screen.height,
      }),
    }

    // console.log(requestBody)

    const response = await fetch('/api/auth/login', requestBody)

    if (response.ok) {
      const responseJson = await response.json()
      const parsedToken = parseJwt(responseJson.access_token) as AccessTokenClaims
      const permissions = new Permissions(parsedToken.permissions)
      context.setAccessToken(responseJson.access_token)
      context.setSession({
        userId: parsedToken.sub,
        expiresOnUTC: parsedToken.exp,
        permissions: permissions.permissions,
        hasPermission: permissions.hasPermission,
        can: permissions.can,
        isSystemAdministrator: permissions.isSystemAdministrator,
      })
      localStorage.setItem('qview_session', '1')
      return null
    } else {
      context.setAccessToken(undefined)
      context.setSession(undefined)
      const body = await response.json().catch(() => ({}))
      return body.error ?? 'Login failed. Please try again.'
    }
  }

  const logout = async (): Promise<boolean> => {
    const response = await fetch('/api/auth/logout', {
      method: 'POST',
    })

    if (response.ok) {
      context.setAccessToken(undefined)
      context.setSession(undefined)
      localStorage.removeItem('qview_session')
      return true
    } else {
      return false
    }
  }

  // Guarantees an access token is seeded (from the refresh cookie) when one is missing —
  // used by auth-gated pages so a fresh login / reload reliably produces a usable token.
  const refresh = useCallback(() => seedAccessToken(context), [context])

  return {
    accessToken: context.accessToken,
    session: context.session,
    isCheckingAuth: context.isCheckingAuth,
    isAuthenticated: !!context.accessToken,
    login,
    logout,
    refresh,
  }
}

export const useAuthCheck = () => {
  const context = useContext(Context)
  const location = useLocation()

  const refreshIfNecessary = useCallback(async () => {
    context.setCheckingAuth(true)

    const isExpiringSoon = () => {
      if (context.session?.expiresOnUTC) {
        const expireTimeMS = context.session.expiresOnUTC * 1000
        const currentTimeMS = Date.now()

        return expireTimeMS - currentTimeMS <= REMAINING_TOKEN_EXPIRY_TIME_ALLOWED
      }

      return true
    }

    if (!context.accessToken || isExpiringSoon()) {
      await seedAccessToken(context)
    }

    context.setCheckingAuth(false)
  }, [context.accessToken, context.session])

  useEffect(() => {
    refreshIfNecessary()
    let intervalId: number | undefined = undefined

    if (context.accessToken) {
      // if the access token is set, we want to check its expiry on some interval
      intervalId = setInterval(() => {
        refreshIfNecessary()
      }, MILLISECONDS_UNTIL_EXPIRY_CHECK)
    }
    return () => {
      if (intervalId) clearInterval(intervalId)
    }
  }, [refreshIfNecessary])

  // Re-check on every client-side navigation
  useEffect(() => {
    refreshIfNecessary()
  }, [location.pathname])

  // Re-check when the user returns to this tab
  useEffect(() => {
    const onVisible = () => {
      if (document.visibilityState === 'visible') refreshIfNecessary()
    }
    document.addEventListener('visibilitychange', onVisible)
    return () => document.removeEventListener('visibilitychange', onVisible)
  }, [refreshIfNecessary])
}

// https://stackoverflow.com/a/38552302
const parseJwt = (token: string) => {
  const base64Url = token.split('.')[1]
  const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/')
  const jsonPayload = decodeURIComponent(
      atob(base64)
          .split('')
          .map(function (c) {
            return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2)
          })
          .join('')
  )

  return JSON.parse(jsonPayload)
}
