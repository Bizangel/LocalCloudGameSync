import { useCallback, useEffect, useState } from "react";
import { useControllerEvent } from "./useControllerEvent";
import { BUTTONS, type Direction } from "../gamepad/common";
import { useLeftStickEvent } from "./useLeftStickEvent";
import { useKeyboardNavigation } from "./useKeyboardArrowsEvent";
import { useKeyboardPress } from "./useKeyboardPress";

type ConfirmCallback = (idx: number) => void;

export type NavigationOrientation = "horizontal" | "vertical";

export function useMultiInputNavigation(
    indexCount: number,
    confirmationCallback: ConfirmCallback,
    cancelCallback?: () => void,
    enabled: boolean = true,
    orientation: NavigationOrientation = "horizontal"
)
: number | null
{
    const [navIndex, setNavIndex] = useState<null | number>(null);
    useEffect(() => {
        if (!enabled)
            setNavIndex(null); // unselect if disabling
    }, [enabled, setNavIndex])

    const onConfirmation = useCallback(() => {
        if (enabled && navIndex !== null)
            confirmationCallback(navIndex);
    }, [enabled, navIndex, confirmationCallback])

    const onCancel = useCallback(() => {
        if (enabled)
            cancelCallback?.();
    }, [enabled, cancelCallback])

    const moveIdxCallback = useCallback((dir: Direction) => {
        if (!enabled || indexCount <= 0) return;

        if (orientation === "horizontal") {
            if (dir === "LEFT") {
                setNavIndex(idx => idx !== null ? ((idx - 1 + indexCount) % indexCount) : 0);
            }
            if (dir === "RIGHT") {
                setNavIndex(idx => idx !== null ? ((idx + 1) % indexCount) : 1 % indexCount);
            }
        } else if (orientation === "vertical") {
            if (dir === "UP") {
                setNavIndex(idx => idx !== null ? ((idx - 1 + indexCount) % indexCount) : (indexCount - 1));
            }
            if (dir === "DOWN") {
                setNavIndex(idx => idx !== null ? ((idx + 1) % indexCount) : 0);
            }
        }
    }, [enabled, indexCount, orientation])

    // Confirm with both controller and keyboard
    useControllerEvent(ev => {
        if (ev.type === "buttonRelease") {
            if (ev.button === BUTTONS.A) {
                onConfirmation();
            }
        }
    }, [onConfirmation])

    useKeyboardPress((key) => {
        if (key == "Enter") {
            onConfirmation();
        }
    }, [onConfirmation])

    // Allow cancelling with both keyboard ESC and gamepad B
     useControllerEvent(ev => {
        if (ev.type === "buttonRelease") {
            if (ev.button === BUTTONS.B) {
                onCancel();
            }
        }
    }, [onConfirmation])

    useKeyboardPress((key) => {
        if (key == "Escape") {
            onCancel();
        }
    }, [onConfirmation])

    // Move with both keyboard and gamepad
    useLeftStickEvent((dir) => {
        moveIdxCallback(dir);
    }, [moveIdxCallback])

    useKeyboardNavigation((dir) => {
        moveIdxCallback(dir)
    })

    return navIndex
}
