import './ErrorDisplay.css'

type ErrorDisplayProps = {
  error: { title: string; subtext: string }
  onContinue?: () => void
  onClose?: () => void
  onRetry?: () => void
}

const ErrorDisplay = ({ error: { title, subtext }, onContinue, onClose, onRetry }: ErrorDisplayProps) => {
  return (
    <div className="container">
      <div className="error-wrapper">
        <div className="error-icon">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 64 64"
            width="64"
            height="64"
            fill="none"
          >
            <circle cx="32" cy="32" r="30" stroke="#e74c3c" strokeWidth="4" />
            <line
              x1="20"
              y1="20"
              x2="44"
              y2="44"
              stroke="#e74c3c"
              strokeWidth="4"
              strokeLinecap="round"
            />
            <line
              x1="44"
              y1="20"
              x2="20"
              y2="44"
              stroke="#e74c3c"
              strokeWidth="4"
              strokeLinecap="round"
            />
          </svg>
        </div>

        <h1>{title}</h1>
        <p>{subtext}</p>

        <div className="error-buttons">
          <button className="btn danger" onClick={onContinue}>
            Continue Anyways
          </button>
          <button className="btn secondary" onClick={onClose}>
            Close
          </button>
          <button className="btn neutral" onClick={onRetry}>
            Retry
          </button>
        </div>
      </div>
    </div>
  )
}

export default ErrorDisplay
