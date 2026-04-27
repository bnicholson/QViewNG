import React, { createContext, useCallback, useContext, useEffect, useState } from 'react'
import { useLocation } from 'react-router-dom'

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

class Permissions {
  private readonly rolesSet: Set<string>
  private readonly rolesArray: string[]

  private readonly permissionsSet: Set<string>
  private readonly permissionsArray: string[]

  constructor(roles: string[], perms: string[]) {
    this.rolesArray = roles
    this.permissionsArray = perms

    this.rolesSet = new Set(this.rolesArray)
    this.permissionsSet = new Set(this.permissionsArray)
  }

  public get roles(): string[] {
    return this.rolesArray
  }

  public get permissions(): string[] {
    return this.permissionsArray
  }

  public hasRole = (role: string): boolean => {
    return this.rolesSet.has(role)
  }

  public hasPermission = (permission: string): boolean => {
    return this.permissionsSet.has(permission)
  }
}

interface Session {
  expiresOnUTC: number
  userId: ID
  roles: string[]
  permissions: string[]
  hasRole(role: string): boolean
  hasPermission(permission: string): boolean
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
      const permissions = new Permissions(parsedToken.roles, parsedToken.permissions)
      context.setAccessToken(responseJson.access_token)
      context.setSession({
        userId: parsedToken.sub,
        expiresOnUTC: parsedToken.exp,
        roles: permissions.roles,
        permissions: permissions.permissions,
        hasPermission: permissions.hasPermission,
        hasRole: permissions.hasRole,
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

  return {
    accessToken: context.accessToken,
    session: context.session,
    isCheckingAuth: context.isCheckingAuth,
    isAuthenticated: !!context.accessToken,
    login,
    logout,
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
      if (!localStorage.getItem('qview_session')) {
        context.setCheckingAuth(false)
        return
      }

      const response = await fetch('/api/auth/refresh', {
        method: 'POST',
      })

      if (response.ok) {
        const responseJson = await response.json()
        const parsedToken = parseJwt(responseJson.access_token) as AccessTokenClaims
        const permissions = new Permissions(parsedToken.roles, parsedToken.permissions)

        context.setAccessToken(responseJson.access_token)
        context.setSession({
          userId: parsedToken.sub,
          expiresOnUTC: parsedToken.exp,
          roles: permissions.roles,
          permissions: permissions.permissions,
          hasRole: permissions.hasRole,
          hasPermission: permissions.hasPermission,
        })
      } else {
        context.setAccessToken(undefined)
        context.setSession(undefined)
        localStorage.removeItem('qview_session')
      }
    } else {
      // console.log(`${context.accessToken ? 'access token' : ''} ${isExpiringSoon() ? ' is not expiring' : ''}`)
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
