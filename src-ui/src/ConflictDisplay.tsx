import { useCallback, useMemo, useState } from 'react'
import './ConflictDisplay.css'
import { useMultiInputNavigation } from './hooks/useMultiInputNavigation'
import { ConfirmModal } from './ConfirmModal'

type SelectionKey = 'remote' | 'local'

type ConflictDisplayProps = {
  conflict: { localModified: string; remoteModified: string }
  onChooseLocal: () => void
  onChooseRemote: () => void
}

const ConflictDisplay = ({
  conflict: { localModified, remoteModified },
  onChooseLocal,
  onChooseRemote,
}: ConflictDisplayProps) => {
  const options = useMemo(
    () => [
      {
        key: 'remote' as const,
        perform: onChooseRemote,
        confirmTitle: 'Use Remote Save?',
        confirmDescription: 'This will overwrite the local save data with the remote version stored on the server.',
        confirmLabel: 'Keep Remote Save',
        confirmClassName: 'neutral',
      },
      {
        key: 'local' as const,
        perform: onChooseLocal,
        confirmTitle: 'Use Local Save?',
        confirmDescription: 'This will overwrite the server copy with your local save data.',
        confirmLabel: 'Keep Local Save',
        confirmClassName: 'neutral',
      },
    ],
    [onChooseLocal, onChooseRemote]
  )

  const [pendingSelection, setPendingSelection] = useState<SelectionKey | null>(null)
  const pendingOption = useMemo(
    () => options.find((option) => option.key === pendingSelection) ?? null,
    [options, pendingSelection]
  )

  const onSelectionClick = useCallback((selection: SelectionKey) => {
    setPendingSelection(selection)
  }, [])

  const handleActivation = useCallback(
    (idx: number) => {
      const option = options[idx]
      if (option) {
        setPendingSelection(option.key)
      }
    },
    [options]
  )

  const focusedIndex = useMultiInputNavigation(
    options.length,
    handleActivation,
    undefined,
    pendingSelection === null && options.length > 0,
    'vertical'
  )

  const handleConfirmSelection = useCallback(() => {
    if (!pendingSelection) return

    const option = options.find((entry) => entry.key === pendingSelection)
    option?.perform()
    setPendingSelection(null)
  }, [options, pendingSelection])

  const handleCancelSelection = useCallback(() => {
    setPendingSelection(null)
  }, [])

  return (
    <div className="container">
      <div className="conflict-wrapper">
        <div className="conflict-icon-wrapper">
          <svg
            className="warning-icon"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 120 120"
            width="64"
            height="64"
          >
            <path
              d="M60 12L8 108h104L60 12z"
              stroke="#f1c40f"
              strokeWidth="8"
              strokeLinejoin="round"
              fill="none"
            />
            <rect x="56" y="44" width="8" height="34" rx="4" fill="#f1c40f" />
            <circle cx="60" cy="88" r="6" fill="#f1c40f" />
          </svg>
        </div>

        <h1>Sync Conflict</h1>

        <div className="conflict-subtexts">
          <p className="conflict-subtext">
            Local changes conflict with save data on the remote. Unable to automatically determine which save version to keep.
          </p>
          <p className="conflict-subtext">Choose a version to keep.</p>
        </div>

        <div className="conflict-options">
          <div
            className={`conflict-card${focusedIndex === 0 ? ' focused' : ''}`}
            onClick={() => onSelectionClick('remote')}
            tabIndex={-1}
            role="button"
            aria-selected={focusedIndex === 0}
          >
            <div className="conflict-card-inner">
              <div className="conflict-icon">
                <svg xmlns="http://www.w3.org/2000/svg" width="36" height="36" fill="none" viewBox="0 0 24 24">
                  <path
                    d="M12 3a6 6 0 0 0-5.9 5.1A4 4 0 0 0 7 16h10a4 4 0 0 0 0-8h-.26A6 6 0 0 0 12 3z"
                    stroke="#3498db"
                    strokeWidth="2"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    fill="none"
                  />
                </svg>
              </div>
              <div className="conflict-info">
                <h2>Remote Save</h2>
                <p>Uploaded on {remoteModified}</p>
              </div>
            </div>
          </div>

          <div
            className={`conflict-card${focusedIndex === 1 ? ' focused' : ''}`}
            onClick={() => onSelectionClick('local')}
            tabIndex={-1}
            role="button"
            aria-selected={focusedIndex === 1}
          >
            <div className="conflict-card-inner">
              <div className="conflict-icon">
                <svg xmlns="http://www.w3.org/2000/svg" width="36" height="36" fill="none" viewBox="0 0 24 24">
                  <path
                    d="M4 4h16v16H4z"
                    stroke="#e67e22"
                    strokeWidth="2"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    fill="none"
                  />
                  <path
                    d="M4 9h16"
                    stroke="#e67e22"
                    strokeWidth="2"
                    strokeLinecap="round"
                  />
                </svg>
              </div>
              <div className="conflict-info">
                <h2>Local Save</h2>
                <p>Modified {localModified}</p>
              </div>
            </div>
          </div>
        </div>

        <p className="conflict-note">The option you choose not to keep will be discarded.</p>
      </div>

      {pendingOption && (
        <ConfirmModal
          onConfirm={handleConfirmSelection}
          onCancel={handleCancelSelection}
          title={pendingOption.confirmTitle}
          description={pendingOption.confirmDescription}
          confirmLabel={pendingOption.confirmLabel}
          confirmClassName={pendingOption.confirmClassName}
          cancelLabel="Cancel"
          cancelClassName="secondary"
        />
      )}
    </div>
  )
}

export default ConflictDisplay
