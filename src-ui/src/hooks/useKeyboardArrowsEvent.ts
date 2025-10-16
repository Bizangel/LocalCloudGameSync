import { useEffect, useRef } from "react";
import type { Direction, DirectionCallback } from "../gamepad/common";

// Keyboard navigation hook
export function useKeyboardNavigation(
  callback: DirectionCallback,
  deps: React.DependencyList = []
): void {
  const callbackRef = useRef<DirectionCallback>(callback);
  const pressedKeys = useRef<Set<string>>(new Set());

  useEffect(() => {
    callbackRef.current = callback;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [callback, ...deps]);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent): void => {
      // Prevent multiple events from key repeat
      if (pressedKeys.current.has(event.key)) {
        return;
      }

      let direction: Direction | null = null;

      switch (event.key) {
        case 'ArrowUp':
          direction = 'UP';
          break;
        case 'ArrowDown':
          direction = 'DOWN';
          break;
        case 'ArrowLeft':
          direction = 'LEFT';
          break;
        case 'ArrowRight':
          direction = 'RIGHT';
          break;
      }

      if (direction) {
        pressedKeys.current.add(event.key);
        callbackRef.current(direction);
      }
    };

    const handleKeyUp = (event: KeyboardEvent): void => {
      pressedKeys.current.delete(event.key);
    };

    window.addEventListener('keydown', handleKeyDown);
    window.addEventListener('keyup', handleKeyUp);

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
      pressedKeys.current.clear();
    };
  }, []);
}