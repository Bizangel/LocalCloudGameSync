import { useContext } from "react";
import { GamepadContext } from "../gamepad/gamepadContext";


// Hook to get gamepad status
export function useGamepadStatus(): { connected: boolean; gamepadIndex: number | null } {
  const context = useContext(GamepadContext);
  if (!context) {
    throw new Error('useGamepadStatus must be used within GamepadProvider');
  }

  return {
    connected: context.connected,
    gamepadIndex: context.gamepadIndex,
  };
}