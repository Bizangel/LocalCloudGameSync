// Gamepad button indices
export const BUTTONS = {
  A: 0,
  B: 1,
  X: 2,
  Y: 3,
  LB: 4,
  RB: 5,
  LT: 6,
  RT: 7,
  SELECT: 8,
  START: 9,
  L3: 10,
  R3: 11,
  DPAD_UP: 12,
  DPAD_DOWN: 13,
  DPAD_LEFT: 14,
  DPAD_RIGHT: 15,
} as const;

// Event types
export const CONTROLLER_EVENTS = {
  BUTTON_PRESS: 'buttonPress',
  BUTTON_RELEASE: 'buttonRelease',
  AXIS_MOVE: 'axisMove',
} as const;

// Type definitions
export type ButtonIndex = typeof BUTTONS[keyof typeof BUTTONS];
export type ControllerEventType = typeof CONTROLLER_EVENTS[keyof typeof CONTROLLER_EVENTS];

export interface ControllerEvent {
  type: ControllerEventType;
  button: number;
  gamepadIndex: number;
  timestamp: number;
}

export type ControllerEventCallback = (event: ControllerEvent) => void;

export interface GamepadContextValue {
  connected: boolean;
  gamepadIndex: number | null;
  addEventListener: (callback: ControllerEventCallback) => () => void;
}

export type ButtonStates = {
  [key: number]: boolean;
}