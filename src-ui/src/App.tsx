import { useCallback, useState } from 'react'
import './App.css'
import { useWebViewEvent } from './hooks/useGlobalRustEventListener'
import { type WebViewState } from './ipc/common';
import LoadingDisplay from './LoadingDisplay';

function App() {
  const [webViewState, setWebViewState] = useState<WebViewState>("Loading");
  const [display, setDisplay] = useState({ title: "Loading", subtext: "" });

  useWebViewEvent("WebViewStateChange", useCallback((ev) => {
    setWebViewState(ev.state);
  }, [setWebViewState]));

  useWebViewEvent("WebViewUpdate", useCallback((ev) => {
    setDisplay({ title: ev.title_text, subtext: ev.sub_text });
  }, [setDisplay]));

  switch (webViewState) {
    case "Loading":
      return <LoadingDisplay {...{display}}/>
    case "Error":
      return null;
    case "Conflict":
      return null;
  }
}

export default App;
