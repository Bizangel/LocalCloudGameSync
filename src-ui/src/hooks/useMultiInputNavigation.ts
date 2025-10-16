import { useCallback, useState } from "react";
import { useControllerEvent } from "./useControllerEvent";
import { BUTTONS, type Direction } from "../gamepad/common";
import { useLeftStickEvent } from "./useLeftStickEvent";
import { useKeyboardNavigation } from "./useKeyboardArrowsEvent";
import { useKeyboardPress } from "./useKeyboardPress";

type ConfirmCallback = (idx: number) => void;

export function useMultiInputNavigation(indexCount: number, confirmationCallback: ConfirmCallback) {
    const [navIndex, setNavIndex] = useState(0);

    const onConfirmation = useCallback(() => {
        confirmationCallback(navIndex);
    }, [navIndex])

    const moveIdxCallback = useCallback((dir: Direction) => {
        if (dir == "LEFT")
            setNavIndex(idx => ((idx - 1 + indexCount) % indexCount));
        if (dir == "RIGHT")
            setNavIndex(idx => ((idx + 1) % indexCount));
    }, [indexCount])

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

    // Move with both keyboard and gamepad
    useLeftStickEvent((dir) => {
        moveIdxCallback(dir);
    }, [moveIdxCallback])

    useKeyboardNavigation((dir) => {
        moveIdxCallback(dir)
    })

    return navIndex
}