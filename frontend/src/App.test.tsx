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
    // This test will need to be updated based on what's actually in your App component
    // This is just a placeholder assertion
    expect(screen.getByRole('heading')).toBeDefined()
  })
})
