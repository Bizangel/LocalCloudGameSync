import { useRef, useState } from 'react'
import './ErrorDisplay.css'
import { useLeftStickEvent } from './hooks/useLeftStickEvent';
import { useKeyboardNavigation } from './hooks/useKeyboardArrowsEvent';

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
  const modalRef = useRef<HTMLDivElement | null>(null)


  useLeftStickEvent((ev) => {
    console.log(ev);
  })

  useKeyboardNavigation((ev) => {
    console.log(ev);
  })

  const baseButtons = [
    { label: 'Continue Anyways', className: 'danger', action: () => setShowConfirm(true) },
    { label: 'Close', className: 'secondary', action: onClose },
    { label: 'Retry', className: 'neutral', action: onRetry },
  ]

  const modalButtons = [
    { label: 'Cancel', className: 'secondary', action: () => setShowConfirm(false) },
    {
      label: 'Yes, Continue',
      className: 'danger',
      action: () => {
        setShowConfirm(false)
        onContinue?.()
      },
    },
  ]

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
              className={`btn ${btn.className} ${0 === i ? 'focused' : ''}`}
              onClick={btn.action}
            >
              {btn.label}
            </button>
          ))}
        </div>
      </div>

      {showConfirm && (
        <div className="modal-backdrop">
          <div className="modal" ref={modalRef}>
            <h2>Are you sure?</h2>
            <p>Continuing may cause data loss or other issues. Do you really want to proceed?</p>
            <div className="modal-buttons">
              {modalButtons.map((btn, i) => (
                <button
                  key={btn.label}
                  className={`btn ${btn.className} ${0 === i ? 'focused' : ''}`}
                  onClick={btn.action}
                >
                  {btn.label}
                </button>
              ))}
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

export default ErrorDisplay
