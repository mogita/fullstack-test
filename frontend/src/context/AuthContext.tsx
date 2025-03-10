import { createContext, useContext, useEffect, useState, ReactNode } from 'react'
import axios from 'axios'
import { jwtDecode } from 'jwt-decode'

// Define types
interface User {
  username: string
}

interface AuthContextType {
  isAuthenticated: boolean
  user: User | null
  login: (username: string, password: string) => Promise<void>
  logout: () => void
  loading: boolean
  error: string | null
}

// Set base URL for API calls
const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3001'

// Configure axios to send credentials with cross-origin requests
axios.defaults.withCredentials = true

// Create context
const AuthContext = createContext<AuthContextType | undefined>(undefined)

// Helper function to set a cookie with token
const setAuthCookie = (token: string) => {
  // Set the cookie to expire in 30 days
  const expiresIn = 30 * 24 * 60 * 60 // 30 days in seconds
  const expires = new Date(Date.now() + expiresIn * 1000)

  // For localhost development across different ports, we need SameSite=None
  // In production, you'd use Secure flag as well, but for local development we can omit it
  document.cookie = `auth_token=${token}; expires=${expires.toUTCString()}; path=/; SameSite=None`
}

// Helper function to clear the auth cookie
const clearAuthCookie = () => {
  document.cookie = 'auth_token=; expires=Thu, 01 Jan 1970 00:00:00 GMT; path=/; SameSite=None'
}

// Provider component
export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [user, setUser] = useState<User | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  // Check for existing token on initial load
  useEffect(() => {
    const checkAuth = async () => {
      try {
        const token = localStorage.getItem('token')
        if (token) {
          // Decode token to get user info
          const decoded = jwtDecode<{ sub: string }>(token)
          setUser({ username: decoded.sub })

          // Ensure cookie is also set
          setAuthCookie(token)
        }
      } catch (err) {
        // If token is invalid, clear it
        localStorage.removeItem('token')
        clearAuthCookie()
        setUser(null)
      } finally {
        setLoading(false)
      }
    }

    checkAuth()
  }, [])

  // Login function
  const login = async (username: string, password: string) => {
    try {
      setError(null)
      setLoading(true)

      const response = await axios.post(`${API_URL}/api/auth/login`, {
        username,
        password,
      })

      const { token } = response.data

      // Store token in localStorage for persistence
      localStorage.setItem('token', token)

      // Also store token in cookie for SSE requests
      setAuthCookie(token)

      // Decode token to get user info
      const decoded = jwtDecode<{ sub: string }>(token)
      setUser({ username: decoded.sub })
    } catch (err) {
      if (axios.isAxiosError(err) && err.response) {
        setError(err.response.data.message || 'Invalid username or password')
      } else {
        setError('An error occurred during login')
      }
      throw err
    } finally {
      setLoading(false)
    }
  }

  // Logout function
  const logout = () => {
    localStorage.removeItem('token')
    clearAuthCookie()
    setUser(null)
  }

  // Context value
  const value = {
    isAuthenticated: !!user,
    user,
    login,
    logout,
    loading,
    error,
  }

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
}

// Custom hook to use auth context
export const useAuth = (): AuthContextType => {
  const context = useContext(AuthContext)
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider')
  }
  return context
}
