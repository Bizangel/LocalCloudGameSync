import { useContext, useEffect, useRef } from "react";
import type { ControllerEventCallback } from "../gamepad/common";
import { GamepadContext } from "../gamepad/gamepadContext";

// Hook to use controller events
export function useControllerEvent(callback: ControllerEventCallback, deps: React.DependencyList = []): void {
  const context = useContext(GamepadContext);

  if (!context) {
    throw new Error('useControllerEvent must be used within GamepadProvider');
  }

  const callbackRef = useRef<ControllerEventCallback>(callback);

  // Update callback ref when deps change
  useEffect(() => {
    callbackRef.current = callback;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [callback, ...deps]);

  useEffect(() => {
    const wrappedCallback: ControllerEventCallback = (event) => {
      callbackRef.current(event);
    };

    return context.addEventListener(wrappedCallback);
  }, [context]);
}