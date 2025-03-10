import { describe, it, expect, vi } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import LoginPage from './LoginPage'

// Create a mock login function
const mockLogin = vi.fn().mockImplementation((username, password) => {
  if (username === 'neo' && password === 'script-chairman-fondly-yippee') {
    return Promise.resolve()
  }
  return Promise.reject(new Error('Invalid credentials'))
})

// Mock the hooks and router
vi.mock('../context/AuthContext', () => ({
  useAuth: () => ({
    login: mockLogin,
    loading: false,
    error: null,
  }),
}))

vi.mock('../context/ThemeContext', () => ({
  useTheme: () => ({
    theme: 'light',
    toggleTheme: vi.fn(),
  }),
}))

vi.mock('react-router-dom', () => ({
  useNavigate: () => vi.fn(),
}))

vi.mock('../components/Layout', () => ({
  default: ({ children }: { children: React.ReactNode }) => <div data-testid="layout">{children}</div>,
}))

describe('LoginPage', () => {
  it('renders login form correctly', () => {
    render(<LoginPage />)

    // Check if form elements are rendered
    expect(screen.getByLabelText(/username/i)).toBeInTheDocument()
    expect(screen.getByLabelText(/password/i)).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /log in/i })).toBeInTheDocument()
  })

  it('shows error message when form is submitted without values', async () => {
    render(<LoginPage />)

    // Submit the form without entering values
    fireEvent.click(screen.getByRole('button', { name: /log in/i }))

    // Check if error message is displayed
    expect(screen.getByText(/please enter both username and password/i)).toBeInTheDocument()
  })

  it('calls login function with entered credentials', async () => {
    render(<LoginPage />)

    // Fill in the form
    fireEvent.change(screen.getByLabelText(/username/i), { target: { value: 'neo' } })
    fireEvent.change(screen.getByLabelText(/password/i), { target: { value: 'script-chairman-fondly-yippee' } })

    // Submit the form
    fireEvent.click(screen.getByRole('button', { name: /log in/i }))

    // Check if login function was called with correct values
    await waitFor(() => {
      expect(mockLogin).toHaveBeenCalledWith('neo', 'script-chairman-fondly-yippee')
    })
  })
})
