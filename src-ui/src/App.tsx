import { useCallback } from 'react'
import './App.css'
import { useGlobalRustEventListener } from './hooks/useGlobalRustEventListener'

type IPCMessage = {
  command: string,
  payload: any,
}

function postIPC(msg: IPCMessage) {
  window.ipc.postMessage(JSON.stringify(msg))
}

function App() {

  useGlobalRustEventListener();

  let onbuttonclick = useCallback(() => {
    postIPC({ command: "sample-command", payload: ""})
  }, [])

  return (
    <div className='container'>
      <div className='centered-display'>

      </div>

      <button onClick={onbuttonclick}>
        click me
      </button>
      </div>
  )
}

export default App
