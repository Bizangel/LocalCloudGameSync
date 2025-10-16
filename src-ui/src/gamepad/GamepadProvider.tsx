import React, { useEffect, useRef, useState, useCallback } from 'react';
import { CONTROLLER_EVENTS, type ButtonStates, type ControllerEvent, type ControllerEventCallback, type GamepadContextValue } from './common';
import { GamepadContext } from './gamepadContext';


// Provider component
export function GamepadProvider({ children }: React.PropsWithChildren) {
  const [connected, setConnected] = useState<boolean>(false);
  const [gamepadIndex, setGamepadIndex] = useState<number | null>(null);
  const eventListeners = useRef<Set<ControllerEventCallback>>(new Set());
  const buttonStates = useRef<ButtonStates>({});
  const animationFrame = useRef<number | null>(null);

  // Add event listener
  const addEventListener = useCallback((callback: ControllerEventCallback): (() => void) => {
    eventListeners.current.add(callback);
    return () => {
      eventListeners.current.delete(callback);
    };
  }, []);

  // Emit event to all listeners
  const emitEvent = useCallback((event: ControllerEvent): void => {
    eventListeners.current.forEach((listener) => {
      listener(event);
    });
  }, []);

  // Poll gamepad state
  const pollGamepad = useCallback((): void => {
    if (!navigator.getGamepads || gamepadIndex === null) {
      return;
    }

    try {
      const gamepads = navigator.getGamepads();
      const gamepad = gamepadIndex !== null ? gamepads[gamepadIndex] : null;

      if (!gamepad) {
        animationFrame.current = requestAnimationFrame(pollGamepad);
        return;
      }

      // Check button states
      gamepad.buttons.forEach((button, index) => {
        const wasPressed = buttonStates.current[index] || false;
        const isPressed = button.pressed;

        if (isPressed && !wasPressed) {
          // Button press event
          emitEvent({
            type: CONTROLLER_EVENTS.BUTTON_PRESS,
            button: index,
            gamepadIndex,
            timestamp: Date.now(),
          });

        } else if (!isPressed && wasPressed) {
          // Button release event
          emitEvent({
            type: CONTROLLER_EVENTS.BUTTON_RELEASE,
            button: index,
            gamepadIndex,
            timestamp: Date.now(),
          });
        }

        buttonStates.current[index] = isPressed;
      });

      animationFrame.current = requestAnimationFrame(pollGamepad);
    } catch (error) {
      console.warn('Error polling gamepad:', error instanceof Error ? error.message : 'Unknown error');
    }
  }, [gamepadIndex, emitEvent]);

  // Handle gamepad connection
  useEffect(() => {
    // Check if Gamepad API is available
    if (!navigator.getGamepads) {
      console.warn('Gamepad API not supported in this environment');
      return;
    }

    const handleConnect = (e: GamepadEvent): void => {
      console.log('Gamepad connected:', e.gamepad);
      setGamepadIndex(e.gamepad.index);
      setConnected(true);
    };

    const handleDisconnect = (e: GamepadEvent): void => {
      console.log('Gamepad disconnected:', e.gamepad);
      if (e.gamepad.index === gamepadIndex) {
        setGamepadIndex(null);
        setConnected(false);
        buttonStates.current = {};
      }
    };

    window.addEventListener('gamepadconnected', handleConnect);
    window.addEventListener('gamepaddisconnected', handleDisconnect);

    // Check for already connected gamepads
    try {
      const gamepads = navigator.getGamepads();
      for (let i = 0; i < gamepads.length; i++) {
        const gamepad = gamepads[i];
        if (gamepad) {
          setGamepadIndex(gamepad.index);
          setConnected(true);
          break;
        }
      }
    } catch (error) {
      console.warn('Unable to access gamepad:', error instanceof Error ? error.message : 'Unknown error');
    }

    return () => {
      window.removeEventListener('gamepadconnected', handleConnect);
      window.removeEventListener('gamepaddisconnected', handleDisconnect);
    };
  }, [gamepadIndex]);

  // Start/stop polling
  useEffect(() => {
    if (connected && gamepadIndex !== null) {
      animationFrame.current = requestAnimationFrame(pollGamepad);
    }

    return () => {
      if (animationFrame.current !== null) {
        cancelAnimationFrame(animationFrame.current);
      }
    };
  }, [connected, gamepadIndex, pollGamepad]);

  const value: GamepadContextValue = {
    connected,
    gamepadIndex,
    addEventListener,
  };

  return (
    <GamepadContext.Provider value={value}>
      {children}
    </GamepadContext.Provider>
  );
}