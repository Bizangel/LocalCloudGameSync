import { useCallback, useMemo, useState } from 'react'
import './ErrorDisplay.css'
import { useMultiInputNavigation } from './hooks/useMultiInputNavigation';
import { ConfirmModal } from './ConfirmModal';

type ErrorDisplayProps = {
  error: { title: string; subtext: string }
  onContinueOffline?: () => void
  onClose?: () => void
  onRetry?: () => void
}

const ErrorDisplay = ({
  error: { title, subtext },
  onContinueOffline,
  onClose,
  onRetry,
}: ErrorDisplayProps) => {
  const [showConfirm, setShowConfirm] = useState(false)

  const baseButtons = useMemo(() => [
      { label: 'Continue Offline', className: 'danger', action: () => setShowConfirm(true) },
      { label: 'Close', className: 'secondary', action: onClose },
      { label: 'Retry', className: 'neutral', action: onRetry },
  ], [onClose, onRetry, setShowConfirm])

  const onConfirm = useCallback((idx: number) => {
    const entry = baseButtons[idx]
    if (entry)
      entry.action?.()
  }, [baseButtons])

  const onModalConfirm = useCallback(() => { setShowConfirm(false);  onContinueOffline?.() }, [setShowConfirm, onContinueOffline])
  const onModalCancel = useCallback(() => {setShowConfirm(false)}, [setShowConfirm])
  const buttonIndex = useMultiInputNavigation(baseButtons.length, onConfirm, undefined, !showConfirm);

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
        <ConfirmModal
          onCancel={onModalCancel}
          onConfirm={onModalConfirm}
          title="Continue Offline?"
          description="Continuing offline can potentially cause a save conflict later on. Do you really want to proceed?"
          confirmLabel="Yes, Continue"
          cancelLabel="Cancel"
          confirmClassName="danger"
          cancelClassName="secondary"
        />
      )}
    </div>
  )
}

export default ErrorDisplay
