import { useEffect, useRef } from "react";

// Keyboard navigation hook
export function useKeyboardPress(
  callback: (key: string) => void,
  deps: React.DependencyList = []
): void {
  const callbackRef = useRef<(key: string) => void>(callback);
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

      pressedKeys.current.add(event.key);
      callbackRef.current(event.key);
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