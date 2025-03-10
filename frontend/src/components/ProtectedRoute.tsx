import React from 'react'
import { Navigate, Outlet } from 'react-router-dom'
import { useAuth } from '../context/AuthContext'

const ProtectedRoute: React.FC = () => {
  const { isAuthenticated, loading } = useAuth()

  // Show a loading state while checking authentication
  if (loading) {
    return <div>Loading...</div>
  }

  // Redirect to login if not authenticated
  if (!isAuthenticated) {
    return <Navigate to="/login" replace />
  }

  // Render the protected route
  return <Outlet />
}

export default ProtectedRoute
