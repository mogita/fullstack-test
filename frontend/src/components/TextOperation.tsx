import React from 'react'
import { useTheme } from '../context/ThemeContext'
import { TextOperation } from '../hooks/useApi'

interface TextOperationButtonProps {
  operation: TextOperation
  label: string
  onClick: () => void
  disabled: boolean
}

const operationColors = {
  paraphrase: {
    light: {
      bg: 'bg-blue-600',
      hoverBg: 'hover:bg-blue-700',
      disabledBg: 'bg-blue-400',
    },
    dark: {
      bg: 'bg-blue-700',
      hoverBg: 'hover:bg-blue-800',
      disabledBg: 'bg-blue-900/50',
    },
  },
  expand: {
    light: {
      bg: 'bg-green-600',
      hoverBg: 'hover:bg-green-700',
      disabledBg: 'bg-green-400',
    },
    dark: {
      bg: 'bg-green-700',
      hoverBg: 'hover:bg-green-800',
      disabledBg: 'bg-green-900/50',
    },
  },
  summarize: {
    light: {
      bg: 'bg-purple-600',
      hoverBg: 'hover:bg-purple-700',
      disabledBg: 'bg-purple-400',
    },
    dark: {
      bg: 'bg-purple-700',
      hoverBg: 'hover:bg-purple-800',
      disabledBg: 'bg-purple-900/50',
    },
  },
  translate: {
    light: {
      bg: 'bg-orange-600',
      hoverBg: 'hover:bg-orange-700',
      disabledBg: 'bg-orange-400',
    },
    dark: {
      bg: 'bg-orange-700',
      hoverBg: 'hover:bg-orange-800',
      disabledBg: 'bg-orange-900/50',
    },
  },
}

const TextOperationButton: React.FC<TextOperationButtonProps> = ({ operation, label, onClick, disabled }) => {
  const { theme } = useTheme()
  const colors = operationColors[operation][theme === 'dark' ? 'dark' : 'light']

  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className={`
        px-4 py-2 rounded-md text-white font-medium
        transition-colors duration-200
        focus:outline-none focus:ring-2 focus:ring-offset-2
        ${disabled ? `${colors.disabledBg} cursor-not-allowed` : `${colors.bg} ${colors.hoverBg}`}
        ${theme === 'dark' ? 'focus:ring-offset-gray-800' : 'focus:ring-offset-white'}
      `}
    >
      {label}
    </button>
  )
}

export default TextOperationButton
