import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import App from './App'

describe('App', () => {
  it('renders without crashing', () => {
    render(<App />)
    expect(document.body).toBeDefined()
  })

  it('contains the expected text', () => {
    render(<App />)
    // This test will need to be updated based on what's actually in your App component
    // This is just a placeholder assertion
    expect(screen.getByRole('heading')).toBeDefined()
  })
})