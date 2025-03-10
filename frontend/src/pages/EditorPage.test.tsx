import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import EditorPage from './EditorPage'

// Mock the hooks and router
const mockProcessText = vi.fn()
const mockTranslateText = vi.fn()
const mockResetOutput = vi.fn()

vi.mock('../hooks/useApi', () => ({
  useApi: () => ({
    processText: mockProcessText,
    translateText: mockTranslateText,
    output: 'Test output',
    isProcessing: false,
    error: null,
    resetOutput: mockResetOutput,
  }),
}))

vi.mock('../context/AuthContext', () => ({
  useAuth: () => ({
    isAuthenticated: true,
  }),
}))

vi.mock('../context/ThemeContext', () => ({
  useTheme: () => ({
    theme: 'light',
  }),
}))

vi.mock('react-router-dom', () => ({
  useNavigate: () => vi.fn(),
}))

vi.mock('../components/Layout', () => ({
  default: ({ children }: { children: React.ReactNode }) => <div data-testid="layout">{children}</div>,
}))

describe('EditorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders editor and output areas', () => {
    render(<EditorPage />)

    // Check if editor and output areas are rendered
    expect(screen.getByPlaceholderText(/enter or paste your text here/i)).toBeInTheDocument()
    expect(screen.getByText(/output will appear here/i)).toBeInTheDocument()
  })

  it('renders operation buttons', () => {
    render(<EditorPage />)

    // Check if all operation buttons are rendered
    expect(screen.getByRole('button', { name: /paraphrase/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /expand/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /summarize/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /translate/i })).toBeInTheDocument()
  })

  it('calls processText when operation button is clicked', () => {
    render(<EditorPage />)

    // Enter text in the editor
    const textArea = screen.getByPlaceholderText(/enter or paste your text here/i)
    fireEvent.change(textArea, { target: { value: 'Test text' } })

    // Click the paraphrase button
    const paraphraseButton = screen.getByRole('button', { name: /paraphrase/i })
    fireEvent.click(paraphraseButton)

    // Check if processText was called with correct parameters
    expect(mockProcessText).toHaveBeenCalledWith('paraphrase', 'Test text')
    expect(mockResetOutput).toHaveBeenCalled()
  })

  it('shows target language select when translate button is clicked', () => {
    render(<EditorPage />)

    // Enter text in the editor
    const textArea = screen.getByPlaceholderText(/enter or paste your text here/i)
    fireEvent.change(textArea, { target: { value: 'Test text' } })

    // Click the translate button
    const translateButton = screen.getByRole('button', { name: /translate/i })
    fireEvent.click(translateButton)

    // Check if language select is shown
    expect(screen.getByText(/to english/i)).toBeInTheDocument()
    expect(screen.getByText(/to spanish/i)).toBeInTheDocument()
  })
})
