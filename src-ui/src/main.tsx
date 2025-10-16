import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import { GamepadProvider } from './gamepad/GamepadProvider.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <GamepadProvider>
      <App />
    </GamepadProvider>
  </StrictMode>,
)
