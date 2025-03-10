import React, { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'
import { useTheme } from '../hooks/useTheme'
import Layout from '../components/Layout'

const LoginPage: React.FC = () => {
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [errorMessage, setErrorMessage] = useState<string | null>(null)

  const { login } = useAuth()
  const { theme } = useTheme()
  const navigate = useNavigate()

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!username || !password) {
      setErrorMessage('Please enter both username and password')
      return
    }

    try {
      setIsSubmitting(true)
      setErrorMessage(null)

      await login(username, password)
      navigate('/editor')
    } catch (error) {
      console.error('Login error:', error)
      setErrorMessage('Invalid username or password')
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <Layout>
      <div className="max-w-md mx-auto mt-10">
        <div
          className={`
          p-6 rounded-lg shadow-md
          ${theme === 'dark' ? 'bg-gray-800' : 'bg-white'}
        `}
        >
          <h1
            className={`
            text-2xl font-bold mb-6 text-center
            ${theme === 'dark' ? 'text-white' : 'text-gray-900'}
          `}
          >
            Log in to AI Text Editor
          </h1>

          {errorMessage && (
            <div className="mb-4 p-3 rounded bg-red-100 border border-red-300 text-red-800">{errorMessage}</div>
          )}

          <form onSubmit={handleSubmit}>
            <div className="mb-4">
              <label
                htmlFor="username"
                className={`block mb-2 font-medium ${theme === 'dark' ? 'text-gray-200' : 'text-gray-700'}`}
              >
                Username
              </label>
              <input
                id="username"
                type="text"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                className={`
                  w-full px-3 py-2 rounded border focus:ring-2 focus:outline-none
                  ${
                    theme === 'dark'
                      ? 'bg-gray-700 border-gray-600 text-white focus:ring-indigo-500'
                      : 'bg-white border-gray-300 text-gray-900 focus:ring-indigo-500'
                  }
                `}
                placeholder="Enter your username"
                disabled={isSubmitting}
              />
            </div>

            <div className="mb-6">
              <label
                htmlFor="password"
                className={`block mb-2 font-medium ${theme === 'dark' ? 'text-gray-200' : 'text-gray-700'}`}
              >
                Password
              </label>
              <input
                id="password"
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                className={`
                  w-full px-3 py-2 rounded border focus:ring-2 focus:outline-none
                  ${
                    theme === 'dark'
                      ? 'bg-gray-700 border-gray-600 text-white focus:ring-indigo-500'
                      : 'bg-white border-gray-300 text-gray-900 focus:ring-indigo-500'
                  }
                `}
                placeholder="Enter your password"
                disabled={isSubmitting}
              />
            </div>

            <button
              type="submit"
              disabled={isSubmitting}
              className={`
                w-full px-4 py-2 text-white font-medium rounded
                focus:outline-none focus:ring-2 focus:ring-offset-2
                transition-colors duration-200
                ${
                  isSubmitting
                    ? 'bg-indigo-400 cursor-not-allowed'
                    : 'bg-indigo-600 hover:bg-indigo-700 focus:ring-indigo-500'
                }
                ${theme === 'dark' ? 'focus:ring-offset-gray-800' : 'focus:ring-offset-white'}
              `}
            >
              {isSubmitting ? 'Logging in...' : 'Log in'}
            </button>
          </form>
        </div>
      </div>
    </Layout>
  )
}

export default LoginPage
