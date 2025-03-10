import React from 'react'
import { useTheme } from '../context/ThemeContext'

const TestBanner: React.FC = () => {
  const { theme } = useTheme()

  return (
    <div
      className={`
      w-full py-2 px-4 text-center text-sm font-medium
      ${theme === 'dark' ? 'bg-amber-600/90 text-white' : 'bg-amber-400/90 text-amber-900'}
      sticky top-0 z-50 backdrop-blur-sm
    `}
    >
      ⚠️ Test Environment - All data may be cleared at any time
    </div>
  )
}

export default TestBanner
