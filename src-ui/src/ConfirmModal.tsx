import { useCallback, useMemo } from "react"
import { useMultiInputNavigation } from "./hooks/useMultiInputNavigation"

type ConfirmModalProps = {
    onConfirm: () => void,
    onCancel: () => void,
    title: string,
    description: string,
    confirmLabel: string,
    cancelLabel: string,
    confirmClassName: string,
    cancelClassName: string,
}

type ButtonEntry = {label: string, className: string, action: () => void}
export function ConfirmModal({
    onConfirm,
    onCancel,
    title,
    description,
    confirmLabel,
    cancelLabel,
    confirmClassName,
    cancelClassName,
}: ConfirmModalProps) {

    const modalButtons: ButtonEntry[] = useMemo(() => [
        { label: cancelLabel, className: cancelClassName, action: onCancel},
        {
            label: confirmLabel, className: confirmClassName, action: onConfirm,
        },
    ], [onCancel, cancelClassName, cancelLabel, confirmClassName, confirmLabel, onConfirm])

    const onButtonClick = useCallback((idx: number) => {
        const entry = modalButtons[idx]
        if (entry)
          entry.action?.()
    }, [modalButtons])

    const buttonIndex = useMultiInputNavigation(modalButtons.length, onButtonClick, onCancel);

    return (
        <div className="modal-backdrop">
          <div className="modal">
            {title && <h2>{title}</h2>}
            {description && <p>{description}</p>}
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
