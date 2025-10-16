import { useCallback, useState } from 'react'
import './ErrorDisplay.css'
import { useMultiInputNavigation } from './hooks/useMultiInputNavigation';
import { ConfirmModal } from './ConfirmModal';

type ErrorDisplayProps = {
  error: { title: string; subtext: string }
  onContinue?: () => void
  onClose?: () => void
  onRetry?: () => void
}

const ErrorDisplay = ({
  error: { title, subtext },
  onContinue,
  onClose,
  onRetry,
}: ErrorDisplayProps) => {
  const [showConfirm, setShowConfirm] = useState(false)

  const baseButtons = [
      { label: 'Continue Anyways', className: 'danger', action: () => setShowConfirm(true) },
      { label: 'Close', className: 'secondary', action: onClose },
      { label: 'Retry', className: 'neutral', action: onRetry },
  ]
  const onConfirm = useCallback((idx: number) => {
    let entry = baseButtons[idx]
    if (entry)
      entry.action?.()
  }, [])

  const onModalConfirm = useCallback(() => { setShowConfirm(false);  onContinue?.() }, [setShowConfirm])
  const onModalCancel = useCallback(() => {setShowConfirm(false)}, [setShowConfirm])
  const buttonIndex = useMultiInputNavigation(baseButtons.length, onConfirm, !showConfirm);

  return (
    <div className="container">
      <div className="error-wrapper">
        <div className="error-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="64" height="64" fill="none">
            <circle cx="32" cy="32" r="30" stroke="#e74c3c" strokeWidth="4" />
            <line x1="20" y1="20" x2="44" y2="44" stroke="#e74c3c" strokeWidth="4" strokeLinecap="round" />
            <line x1="44" y1="20" x2="20" y2="44" stroke="#e74c3c" strokeWidth="4" strokeLinecap="round" />
          </svg>
        </div>

        <h1>{title}</h1>
        <p>{subtext}</p>

        <div className="error-buttons">
          {baseButtons.map((btn, i) => (
            <button
              key={btn.label}
              className={`btn ${btn.className} ${buttonIndex === i ? 'focused' : ''}`}
              onClick={btn.action}
            >
              {btn.label}
            </button>
          ))}
        </div>
      </div>

      {showConfirm && (
        <ConfirmModal onCancel={onModalCancel} onConfirm={onModalConfirm}/>
      )}
    </div>
  )
}

export default ErrorDisplay
