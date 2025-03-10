import React, { ReactNode } from 'react'
import TestBanner from './TestBanner'
import Header from './Header'
import { useTheme } from '../hooks/useTheme'

interface LayoutProps {
  children: ReactNode
}

const Layout: React.FC<LayoutProps> = ({ children }) => {
  const { theme } = useTheme()

  return (
    <div
      className={`
      min-h-screen flex flex-col
      ${theme === 'dark' ? 'bg-gray-900 text-gray-100' : 'bg-gray-50 text-gray-900'}
      transition-colors duration-200
    `}
    >
      <TestBanner />
      <Header />
      <main className="flex-1 container mx-auto px-4 sm:px-6 py-6">{children}</main>
    </div>
  )
}

export default Layout
