import React, { useEffect, useRef, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useTheme } from '../hooks/useTheme'
import { useAuth } from '../hooks/useAuth'
import { useApi, TextOperation, TranslationParams } from '../hooks/useApi'
import Layout from '../components/Layout'
import TextOperationButton from '../components/TextOperation'
import Spinner from '../components/Spinner'

const EditorPage: React.FC = () => {
  const [selectedText, setSelectedText] = useState<string>('')
  const [editorText, setEditorText] =
    useState<string>(`The world is full of mysteries waiting to be unraveled. From the depths of the ocean to the vastness of space, there are countless secrets yet to be discovered. Each day, scientists and explorers venture into the unknown, driven by curiosity and a thirst for knowledge. Their discoveries not only expand our understanding of the universe but also inspire new generations to pursue careers in science and exploration.

As technology advances, our ability to explore and understand the world around us improves significantly. Tools like satellites, drones, and advanced computing systems allow us to gather data and analyze it in ways that were previously unimaginable. This technological progress has led to breakthroughs in fields such as medicine, environmental science, and astronomy. Moreover, it has opened up new possibilities for sustainable development and conservation, helping us to better protect our planet.

Despite the many advancements we have made, there is still much to learn. The natural world is complex and interconnected, and understanding these relationships is crucial for addressing global challenges like climate change and biodiversity loss. By continuing to explore and learn, we can develop innovative solutions to these problems and create a more sustainable future for all. This journey of discovery is ongoing, and it requires collaboration and dedication from individuals across the globe.`)
  const [showTargetLanguageSelect, setShowTargetLanguageSelect] = useState<boolean>(false)
  const [targetLanguage, setTargetLanguage] = useState<'english' | 'spanish'>('english')

  const textAreaRef = useRef<HTMLTextAreaElement>(null)

  const { theme } = useTheme()
  const { isAuthenticated } = useAuth()
  const navigate = useNavigate()

  const { processText, translateText, output, isProcessing, error, resetOutput } = useApi()

  // Redirect to login if not authenticated
  useEffect(() => {
    if (!isAuthenticated) {
      navigate('/login')
    }
  }, [isAuthenticated, navigate])

  // Update selected text when selection changes
  const handleTextSelection = () => {
    if (textAreaRef.current) {
      const start = textAreaRef.current.selectionStart
      const end = textAreaRef.current.selectionEnd

      if (start !== end) {
        setSelectedText(editorText.substring(start, end))
      } else {
        setSelectedText('')
      }
    }
  }

  // Handle text change
  const handleTextChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setEditorText(e.target.value)
  }

  // Handle text operations
  const handleOperation = async (operation: Exclude<TextOperation, 'translate'>) => {
    resetOutput()

    // If text is selected, process only that text, otherwise process all text
    const textToProcess = selectedText || editorText
    if (!textToProcess) return

    await processText(operation, textToProcess)
  }

  // Handle translation
  const handleTranslate = async () => {
    if (showTargetLanguageSelect) {
      resetOutput()

      // If text is selected, translate only that text, otherwise translate all text
      const textToTranslate = selectedText || editorText
      if (!textToTranslate) return

      const params: TranslationParams = {
        text: textToTranslate,
        target_language: targetLanguage,
      }

      await translateText(params)
      setShowTargetLanguageSelect(false)
    } else {
      setShowTargetLanguageSelect(true)
    }
  }

  // Handle target language selection
  const handleTargetLanguageChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setTargetLanguage(e.target.value as 'english' | 'spanish')
  }

  return (
    <Layout>
      <div className="flex flex-col h-[calc(100vh-12rem)] max-w-5xl mx-auto">
        {/* Text Editor */}
        <div className="h-1/2 mb-4">
          <textarea
            ref={textAreaRef}
            value={editorText}
            onChange={handleTextChange}
            onSelect={handleTextSelection}
            onClick={handleTextSelection}
            onKeyUp={handleTextSelection}
            placeholder="Enter or paste your text here..."
            className={`
              w-full h-full p-4 rounded-lg border resize-none font-sans text-base
              focus:outline-none focus:ring-2 focus:ring-indigo-500
              ${
                theme === 'dark'
                  ? 'bg-gray-800 border-gray-700 text-gray-100'
                  : 'bg-white border-gray-300 text-gray-900'
              }
            `}
          />
        </div>

        {/* Controls */}
        <div className="mb-4">
          <div className="flex flex-wrap gap-2">
            <TextOperationButton
              operation="paraphrase"
              label="Paraphrase"
              onClick={() => handleOperation('paraphrase')}
              disabled={isProcessing || !editorText}
            />

            <TextOperationButton
              operation="expand"
              label="Expand"
              onClick={() => handleOperation('expand')}
              disabled={isProcessing || !editorText}
            />

            <TextOperationButton
              operation="summarize"
              label="Summarize"
              onClick={() => handleOperation('summarize')}
              disabled={isProcessing || !editorText}
            />

            {!showTargetLanguageSelect ? (
              <TextOperationButton
                operation="translate"
                label="Translate"
                onClick={handleTranslate}
                disabled={isProcessing || !editorText}
              />
            ) : (
              <div className="flex space-x-2">
                <select
                  value={targetLanguage}
                  onChange={handleTargetLanguageChange}
                  className={`
                    rounded-md border px-3 py-2
                    ${
                      theme === 'dark'
                        ? 'bg-gray-700 border-gray-600 text-white'
                        : 'bg-white border-gray-300 text-gray-900'
                    }
                  `}
                >
                  <option value="english">To English</option>
                  <option value="spanish">To Spanish</option>
                </select>

                <TextOperationButton
                  operation="translate"
                  label="Go"
                  onClick={handleTranslate}
                  disabled={isProcessing || !editorText}
                />
              </div>
            )}
          </div>
        </div>

        {/* Output Area */}
        <div className="relative flex-1">
          <div
            className={`
            w-full h-full p-4 rounded-lg border overflow-auto
            ${theme === 'dark' ? 'bg-gray-800 border-gray-700 text-gray-100' : 'bg-white border-gray-300 text-gray-900'}
          `}
          >
            {isProcessing && (
              <div className="absolute top-2 right-2">
                <Spinner size="sm" />
              </div>
            )}

            {error && (
              <div className="p-3 mb-3 rounded-md bg-red-100 text-red-800 border border-red-300">
                <p className="font-medium">Error: {error}</p>
              </div>
            )}

            <div className="whitespace-pre-wrap">
              {output || (
                <p className={`text-center italic mt-4 ${theme === 'dark' ? 'text-gray-400' : 'text-gray-500'}`}>
                  Output will appear here after processing
                </p>
              )}
            </div>
          </div>
        </div>
      </div>
    </Layout>
  )
}

export default EditorPage
