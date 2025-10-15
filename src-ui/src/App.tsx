import { useCallback, useState } from 'react'
import './App.css'
import { useWebViewEvent } from './hooks/useGlobalRustEventListener'
import { IPC, type WebViewUpdateEvent } from './ipc/common';

function App() {
  const [subtitle, setSubtitle] = useState("")
  const [fadeKey, setFadeKey] = useState(0) // to trigger animation on text change

  const onWebViewChange = useCallback((ev: WebViewUpdateEvent) => {
    console.log("update: ", ev)
    setSubtitle(ev.display_text)
    setFadeKey(prev => prev + 1)
  }, []);

  useWebViewEvent("WebViewUpdate", onWebViewChange);

  return (
    <div className='container'>
      <div className='loading-wrapper'>
        <div className='spinner'></div>
        <h1>Loading</h1>
        <p key={fadeKey} className='fade-text'>{subtitle}</p>
      </div>
    </div>
  )
}

export default App
