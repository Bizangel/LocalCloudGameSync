import { useCallback, useMemo } from "react"
import { useMultiInputNavigation } from "./hooks/useMultiInputNavigation"

type ConfirmModalProps = {
    onConfirm: () => void,
    onCancel: () => void,
}

type ButtonEntry = {label: string, className: string, action: () => void}
export function ConfirmModal({ onConfirm, onCancel }: ConfirmModalProps) {

    const modalButtons: ButtonEntry[] = useMemo(() => [
        { label: 'Cancel', className: 'secondary', action: onCancel},
        {
            label: 'Yes, Continue', className: 'danger', action: onConfirm,
        },
    ], [onConfirm, onCancel])

    const onButtonClick = useCallback((idx: number) => {
        const entry = modalButtons[idx]
        if (entry)
          entry.action?.()
    }, [modalButtons])

    const buttonIndex = useMultiInputNavigation(modalButtons.length, onButtonClick, onCancel);

    return (
        <div className="modal-backdrop">
          <div className="modal">
            <h2>Are you sure?</h2>
            <p>Continuing offline can potentially cause a save conflict later on. Do you really want to proceed?</p>
            <div className="modal-buttons">
              {modalButtons.map((btn, i) => (
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
        </div>
    )
}