import { useEffect, useRef } from "react";
import { useControllerEvent } from "./useControllerEvent";
import { CONTROLLER_EVENTS } from "../gamepad/common";

// Direction types for stick navigation
export type StickDirection = 'UP' | 'DOWN' | 'LEFT' | 'RIGHT';

export interface StickDirectionEvent {
  direction: StickDirection;
  timestamp: number;
}

export type StickDirectionCallback = (event: StickDirectionEvent) => void;

// Hook for left stick navigation with deadzone return requirement
export function useLeftStickEvent(
  callback: StickDirectionCallback,
  deps: React.DependencyList = [],
  threshold: number = 0.5 // Threshold to trigger direction
): void {
  const callbackRef = useRef<StickDirectionCallback>(callback);
  const hasReturnedToDeadzone = useRef<boolean>(true);
  const lastDirection = useRef<StickDirection | null>(null);

  // Update callback ref when deps change
  useEffect(() => {
    callbackRef.current = callback;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [callback, ...deps]);

  useControllerEvent((event) => {
    if (event.type !== CONTROLLER_EVENTS.AXIS_MOVE) {
      return;
    }

    // Left stick X axis (index 0) and Y axis (index 1)
    if (event.axis !== 0 && event.axis !== 1) {
      return;
    }

    const axisValue = event.value;
    const isXAxis = event.axis === 0;

    // Check if stick has returned to deadzone
    if (Math.abs(axisValue) === 0) {
      hasReturnedToDeadzone.current = true;
      lastDirection.current = null;
      return;
    }

    // Only emit event if stick has returned to deadzone since last direction
    if (!hasReturnedToDeadzone.current) {
      return;
    }

    // Check if value exceeds threshold
    if (Math.abs(axisValue) < threshold) {
      return;
    }

    // Determine direction
    let direction: StickDirection | null = null;

    if (isXAxis) {
      direction = axisValue > 0 ? 'RIGHT' : 'LEFT';
    } else {
      // Y axis is inverted (negative = up, positive = down)
      direction = axisValue > 0 ? 'DOWN' : 'UP';
    }

    // Emit event and lock until deadzone return
    if (direction && direction !== lastDirection.current) {
      hasReturnedToDeadzone.current = false;
      lastDirection.current = direction;

      callbackRef.current({
        direction,
        timestamp: event.timestamp,
      });
    }
  }, [threshold]);
}