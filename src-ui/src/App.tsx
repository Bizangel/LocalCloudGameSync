import { useCallback } from 'react'
import './App.css'
import { useWebViewEvent } from './hooks/useGlobalRustEventListener'
import { IPC, type WebViewUpdateEvent } from './ipc/common';

function App() {
  const onWebViewChange = useCallback((ev: WebViewUpdateEvent) => {
    console.log("update: ", ev)
  }, []);
  useWebViewEvent("WebViewUpdate", onWebViewChange);

  const conflictresolve = useCallback(() => {
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
