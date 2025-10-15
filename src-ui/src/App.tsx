import { useCallback, useState } from 'react'
import './App.css'
import { useWebViewEvent } from './hooks/useGlobalRustEventListener'
import { type WebViewUpdateEvent } from './ipc/common';

function App() {
  const [display, setDisplay] = useState({title: "", subtext: ""})
  const [fadeKey, setFadeKey] = useState(0) // to trigger animation on text change

  const onWebViewChange = useCallback((ev: WebViewUpdateEvent) => {
    setDisplay({ title: ev.title_text, subtext: ev.sub_text })
    setFadeKey(prev => prev + 1)
  }, []);

  useWebViewEvent("WebViewUpdate", onWebViewChange);

  return (
    <div className='container'>
      <div className='loading-wrapper'>
        <div className='spinner'></div>
        <h1>{display.title}</h1>
        <p key={fadeKey} className='fade-text'>{display.subtext}</p>
      </div>
    </div>
  )
}

export default App
