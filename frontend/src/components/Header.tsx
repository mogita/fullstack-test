import React from 'react'
import { useAuth } from '../hooks/useAuth'
import { useTheme } from '../hooks/useTheme'
import { HiSun, HiMoon, HiLogout } from 'react-icons/hi'

const Header: React.FC = () => {
  const { isAuthenticated, logout } = useAuth()
  const { theme, toggleTheme } = useTheme()

  return (
    <header
      className={`
      w-full py-3 px-4 sm:px-6 
      flex justify-between items-center 
      border-b 
      ${theme === 'dark' ? 'bg-gray-900 border-gray-700' : 'bg-white border-gray-200'}
      transition-colors duration-200
    `}
    >
      <div className="flex items-center space-x-2">
        <span
          className={`
          text-xl font-semibold 
          ${theme === 'dark' ? 'text-white' : 'text-gray-800'}
        `}
        >
          AI Text Editor
        </span>
      </div>

      <div className="flex items-center space-x-4">
        <button
          aria-label={theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
          onClick={toggleTheme}
          className={`
            p-2 rounded-full 
            ${theme === 'dark' ? 'text-amber-400 hover:bg-gray-800' : 'text-amber-600 hover:bg-gray-100'}
            transition-colors
          `}
        >
          {theme === 'dark' ? <HiSun size={20} /> : <HiMoon size={20} />}
        </button>

        {isAuthenticated && (
          <button
            onClick={logout}
            className={`
              flex items-center space-x-1 px-3 py-2 rounded
              ${theme === 'dark' ? 'bg-red-600 hover:bg-red-700 text-white' : 'bg-red-500 hover:bg-red-600 text-white'}
              transition-colors
            `}
          >
            <HiLogout size={18} />
            <span>Logout</span>
          </button>
        )}
      </div>
    </header>
  )
}

export default Header
