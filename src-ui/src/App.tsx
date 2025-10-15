import { useCallback, useState } from 'react'
import './App.css'
import { useWebViewEvent } from './hooks/useGlobalRustEventListener'
import { type WebViewUpdateEvent } from './ipc/common';

function App() {
  const [display, setDisplay] = useState({ title: "Loading", subtext: "", isError: false });
  const [fadeKey, setFadeKey] = useState(0); // triggers animation on text change

  const onWebViewChange = useCallback((ev: WebViewUpdateEvent) => {
    setDisplay({ title: ev.title_text, subtext: ev.sub_text, isError: true });
    setFadeKey(prev => prev + 1);
  }, []);

  useWebViewEvent("WebViewUpdate", onWebViewChange);

  // Example handlers for the buttons
  const handleRetry = () => {
    console.log("Retry clicked");
    // Add your retry logic here
  };

  const handleClose = () => {
    console.log("Close clicked");
    // Add your close logic here
  };

  const handleContinue = () => {
    console.log("Continue Anyways clicked");
    // Add your continue logic here
  };

  return (
    <div className="container">
      <div className="loading-wrapper">
        {display.isError ? (
          <div className="error-icon">
            <span className="error-x">✖</span>
          </div>
        ) : (
          <div className="spinner"></div>
        )}
        <h1>{display.title}</h1>
        <p key={fadeKey} className="fade-text">{display.subtext}</p>

        {display.isError && (
          <div className="error-buttons">
            <button onClick={handleRetry}>Retry</button>
            <button onClick={handleClose}>Close</button>
            <button onClick={handleContinue}>Continue Anyways</button>
          </div>
        )}
      </div>
    </div>
  )
}

export default App;
