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
export const GAMEPAD_EVENTS = {
  BUTTON_PRESS: 'buttonPress',
  BUTTON_RELEASE: 'buttonRelease',
  AXIS_MOVE: 'axisMove',
} as const;

// Type definitions
export type ButtonIndex = typeof BUTTONS[keyof typeof BUTTONS];
export type GamepadEventType = typeof GAMEPAD_EVENTS[keyof typeof GAMEPAD_EVENTS];

export type AxisMoveEvent = {
  type: typeof GAMEPAD_EVENTS.AXIS_MOVE;
  axis: number;
  value: number;
  gamepadIndex: number;
  timestamp: number;
}

export type ButtonEvent = {
  type: typeof GAMEPAD_EVENTS.BUTTON_PRESS | typeof GAMEPAD_EVENTS.BUTTON_RELEASE;
  button: number;
  gamepadIndex: number;
  timestamp: number;
}

export type PadEvent = ButtonEvent | AxisMoveEvent;
export type PadEventCallback = (event: PadEvent) => void;

export type GamepadContextValue = {
  connected: boolean;
  gamepadIndex: number | null;
  addEventListener: (callback: PadEventCallback) => () => void;
}

export type ButtonStates = {
  [key: number]: boolean;
}

export type AxisStates = {
  [key: number]: number;
}
export type Direction = 'UP' | 'DOWN' | 'LEFT' | 'RIGHT';
export type DirectionCallback = (event: Direction) => void;