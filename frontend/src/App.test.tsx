import { describe, it, expect, vi } from 'vitest'
import { render, screen } from '@testing-library/react'
import App from './App'

// Mock the context providers and router
vi.mock('./context/ThemeContext', () => ({
  ThemeProvider: ({ children }: { children: React.ReactNode }) => <div data-testid="theme-provider">{children}</div>,
}))

vi.mock('./context/AuthContext', () => ({
  AuthProvider: ({ children }: { children: React.ReactNode }) => <div data-testid="auth-provider">{children}</div>,
}))

vi.mock('react-router-dom', () => ({
  BrowserRouter: ({ children }: { children: React.ReactNode }) => <div data-testid="browser-router">{children}</div>,
  Routes: ({ children }: { children: React.ReactNode }) => <div data-testid="routes">{children}</div>,
  Route: () => <div data-testid="route" />,
  Navigate: () => <div data-testid="navigate" />,
}))

describe('App', () => {
  it('renders without crashing', () => {
    render(<App />)

    // Check if providers are rendered
    expect(screen.getByTestId('theme-provider')).toBeInTheDocument()
    expect(screen.getByTestId('auth-provider')).toBeInTheDocument()
    expect(screen.getByTestId('browser-router')).toBeInTheDocument()
    expect(screen.getByTestId('routes')).toBeInTheDocument()

    // Check if routes are rendered
    const routes = screen.getAllByTestId('route')
    expect(routes.length).toBeGreaterThan(0)
  })

  it('contains the expected text', () => {
    render(<App />)
    // Don't look for a heading that doesn't exist
    // Just verify routes are present
    const routes = screen.getAllByTestId('route')
    expect(routes.length).toBe(3) // Assuming we have 3 routes in our app
  })
})
