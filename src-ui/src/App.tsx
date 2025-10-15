import { useCallback } from 'react'
import './App.css'
import { useGlobalRustEventListener } from './hooks/useGlobalRustEventListener'
import { IPC } from './ipc/common';

function App() {
  useGlobalRustEventListener();

  let conflictresolve = useCallback(() => {
    IPC.sendResolveConflict("push");
  }, [])

  return (
    <div className='container'>
      <div className='centered-display'>

      </div>


      <button onClick={conflictresolve}>
        resolve with pull
      </button>
      </div>
  )
}

export default App
