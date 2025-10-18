import { useCallback, useMemo } from 'react'
import './RemoteEmptyDisplay.css'
import { useMultiInputNavigation } from './hooks/useMultiInputNavigation'

type RemoteEmptyDisplayProps = {
  title: string
  subtext: string
  onConfirmPush: () => void
  onCancel: () => void
}

const RemoteEmptyDisplay = ({ title, subtext, onConfirmPush, onCancel }: RemoteEmptyDisplayProps) => {
  const actions = useMemo(
    () => [
      {
        label: 'Confirm Push',
        action: onConfirmPush,
        icon: (
          <svg xmlns="http://www.w3.org/2000/svg" width="36" height="36" viewBox="0 0 24 24" fill="none">
            <path
              d="M4 16v2a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-2"
              stroke="#27ae60"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
            <path
              d="M12 4v12"
              stroke="#27ae60"
              strokeWidth="2"
              strokeLinecap="round"
            />
            <path
              d="M7 9l5-5 5 5"
              stroke="#27ae60"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        ),
      },
      {
        label: 'Close',
        action: onCancel,
        icon: (
          <svg xmlns="http://www.w3.org/2000/svg" width="36" height="36" viewBox="0 0 24 24" fill="none">
            <circle cx="12" cy="12" r="9" stroke="#aaaaaa" strokeWidth="2" />
            <path
              d="M15 9l-6 6"
              stroke="#aaaaaa"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
            <path
              d="M9 9l6 6"
              stroke="#aaaaaa"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        ),
      },
    ],
    [onCancel, onConfirmPush]
  )

  const handleActivation = useCallback(
    (idx: number) => {
      const entry = actions[idx]
      entry?.action()
    },
    [actions]
  )

  const focusedIndex = useMultiInputNavigation(actions.length, handleActivation, onCancel, true, 'vertical')

  return (
    <div className="container">
      <div className="remote-empty-wrapper">
        <div className="remote-empty-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 120" width="64" height="64" className="remote-empty-warning-icon">
            <path
              d="M60 20c20 0 36 6.5 36 14.5S80 49 60 49 24 42.5 24 34.5 40 20 60 20z"
              stroke="#3498db"
              strokeWidth="8"
              fill="none"
            />
            <path
              d="M24 34.5v27c0 8 16 14.5 36 14.5s36-6.5 36-14.5v-27"
              stroke="#3498db"
              strokeWidth="8"
              fill="none"
              strokeLinejoin="round"
            />
            <path
              d="M24 61.5v24c0 8 16 14.5 36 14.5s36-6.5 36-14.5v-24"
              stroke="#3498db"
              strokeWidth="8"
              fill="none"
              strokeLinejoin="round"
            />
            <line x1="60" y1="54" x2="60" y2="72" stroke="#3498db" strokeWidth="7" strokeLinecap="round" />
            <circle cx="60" cy="86" r="5" fill="#3498db" />
          </svg>
        </div>

        <h1>{title}</h1>

        <div className="remote-empty-subtexts">
          <p className="remote-empty-subtext">{subtext}</p>
        </div>

        <div className="remote-empty-options">
          {actions.map((entry, index) => (
            <div
              key={entry.label}
              className={`remote-empty-card${focusedIndex === index ? ' focused' : ''}`}
              onClick={entry.action}
              tabIndex={-1}
              role="button"
              aria-selected={focusedIndex === index}
            >
              <div className="remote-empty-card-inner">
                <div className="remote-empty-icon-wrapper">{entry.icon}</div>
                <div className="remote-empty-info">
                  <h2>{entry.label}</h2>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}

export default RemoteEmptyDisplay
