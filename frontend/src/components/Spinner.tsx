import React from 'react'
import { useTheme } from '../hooks/useTheme'

interface SpinnerProps {
  size?: 'sm' | 'md' | 'lg'
}

const Spinner: React.FC<SpinnerProps> = ({ size = 'md' }) => {
  const { theme } = useTheme()

  const sizeClass = {
    sm: 'w-4 h-4',
    md: 'w-6 h-6',
    lg: 'w-8 h-8',
  }[size]

  return (
    <div className="flex justify-center items-center">
      <div
        className={`
          ${sizeClass} 
          border-4 
          rounded-full 
          animate-spin 
          ${theme === 'dark' ? 'border-gray-600 border-t-indigo-500' : 'border-gray-200 border-t-indigo-600'}
        `}
      />
    </div>
  )
}

export default Spinner
