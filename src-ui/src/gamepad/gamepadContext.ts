import { createContext } from "react";
import type { GamepadContextValue } from "./common";

// Create the context
export const GamepadContext = createContext<GamepadContextValue | null>(null);
