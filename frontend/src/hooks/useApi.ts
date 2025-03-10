import { useState } from 'react'
import axios from 'axios'

// Set base URL for API calls
const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3001'

// Define the types of operations that can be performed
export type TextOperation = 'paraphrase' | 'expand' | 'summarize' | 'translate'

// Type for translation operations
export interface TranslationParams {
  text: string
  target_language: 'english' | 'spanish'
}

interface UseApiReturn {
  processText: (operation: Exclude<TextOperation, 'translate'>, text: string) => Promise<void>
  translateText: (params: TranslationParams) => Promise<void>
  output: string
  isProcessing: boolean
  error: string | null
  resetOutput: () => void
}

export const useApi = (): UseApiReturn => {
  const [output, setOutput] = useState<string>('')
  const [isProcessing, setIsProcessing] = useState<boolean>(false)
  const [error, setError] = useState<string | null>(null)

  // Helper to get auth token
  const getToken = (): string | null => {
    return localStorage.getItem('token')
  }

  // Reset output state
  const resetOutput = () => {
    setOutput('')
    setError(null)
  }

  // Process text with the specified operation
  const processText = async (operation: Exclude<TextOperation, 'translate'>, text: string): Promise<void> => {
    const token = getToken()
    if (!token) {
      setError('Authentication required')
      return
    }

    let eventSource: EventSource | null = null

    try {
      setError(null)
      setIsProcessing(true)
      setOutput('')

      // Create EventSource for SSE - with withCredentials to send cookies
      const url = `${API_URL}/api/text/${operation}?text=${encodeURIComponent(text)}`
      // Use a properly typed EventSource with withCredentials
      const eventSourceInit: EventSourceInit = { withCredentials: true }
      eventSource = new EventSource(url, eventSourceInit)

      // Handle events
      eventSource.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data)
          setOutput((prev) => prev + data.data)
        } catch {
          // If parsing fails, use the raw event data
          setOutput((prev) => prev + event.data)
        }
      }

      eventSource.onerror = (error) => {
        console.error('SSE error:', error)
        if (eventSource) {
          eventSource.close()
        }
        setError('An error occurred while processing your request')
        setIsProcessing(false)
      }

      // Handle when the stream is closed by the server
      eventSource.addEventListener('done', () => {
        if (eventSource) {
          eventSource.close()
        }
        setIsProcessing(false)
      })
    } catch (err) {
      setIsProcessing(false)
      if (axios.isAxiosError(err) && err.response) {
        setError(err.response.data.message || 'An error occurred')
      } else {
        setError('An unexpected error occurred')
      }
      if (eventSource) {
        eventSource.close()
      }
    }
  }

  // Handle translation (has different parameters)
  const translateText = async (params: TranslationParams): Promise<void> => {
    const token = getToken()
    if (!token) {
      setError('Authentication required')
      return
    }

    let eventSource: EventSource | null = null

    try {
      setError(null)
      setIsProcessing(true)
      setOutput('')

      // Create EventSource for SSE - with withCredentials to send cookies
      const url = `${API_URL}/api/text/translate?text=${encodeURIComponent(params.text)}&target_language=${params.target_language}`
      // Use a properly typed EventSource with withCredentials
      const eventSourceInit: EventSourceInit = { withCredentials: true }
      eventSource = new EventSource(url, eventSourceInit)

      // Handle events
      eventSource.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data)
          setOutput((prev) => prev + data.data)
        } catch {
          // If parsing fails, use the raw event data
          setOutput((prev) => prev + event.data)
        }
      }

      eventSource.onerror = (error) => {
        console.error('SSE error:', error)
        if (eventSource) {
          eventSource.close()
        }
        setError('An error occurred while processing your request')
        setIsProcessing(false)
      }

      // Handle when the stream is closed by the server
      eventSource.addEventListener('done', () => {
        if (eventSource) {
          eventSource.close()
        }
        setIsProcessing(false)
      })
    } catch (err) {
      setIsProcessing(false)
      if (axios.isAxiosError(err) && err.response) {
        setError(err.response.data.message || 'An error occurred')
      } else {
        setError('An unexpected error occurred')
      }
      if (eventSource) {
        eventSource.close()
      }
    }
  }

  return {
    processText,
    translateText,
    output,
    isProcessing,
    error,
    resetOutput,
  }
}

// TODO: Implement WebSocket support for real-time collaboration
// TODO: Add rate limiting and offline support
