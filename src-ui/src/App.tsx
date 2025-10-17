import { useCallback, useState } from 'react'
import './App.css'
import { useWebViewEvent } from './hooks/useGlobalRustEventListener'
import { IPC, type WebViewState } from './ipc/common';
import LoadingDisplay from './LoadingDisplay';
import ErrorDisplay from './ErrorDisplay';
import SuccessDisplay from './SuccessDisplay';
import ConflictDisplay from './ConflictDisplay';

function App() {
  const [webViewState, setWebViewState] = useState<WebViewState>("Loading");
  const [display, setDisplay] = useState({ title: "Loading", subtext: "" });

  useWebViewEvent("WebViewStateChange", useCallback((ev) => {
    setWebViewState(ev.state);
  }, [setWebViewState]));

  useWebViewEvent("WebViewUpdate", useCallback((ev) => {
    setDisplay({ title: ev.title_text, subtext: ev.sub_text });
  }, [setDisplay]));

  const closeUnsuccess = useCallback(() => {
    IPC.sendErrorResolve("close");
  }, [])

  const retrySync = useCallback(() => {
    IPC.sendErrorResolve("retry");
  }, [])

  const continueOffline = useCallback(() => {
    IPC.sendErrorResolve("continue-offline");
  }, [])

  switch (webViewState) {
    case "Loading":
      return <LoadingDisplay {...{display}}/>
    case "Error":
      return <ErrorDisplay error={display}
        onClose={closeUnsuccess}
        onRetry={retrySync}
        onContinueOffline={continueOffline}
      />;
    case "Conflict":
      return <ConflictDisplay
        conflict={{
          "localModified": "Thursday, October 21 2021 7:32PM", "remoteModified": "Thursday, October 20 2021 7:00PM"
        }}
      />;
    case "Success":
      return <SuccessDisplay {...{display}} />;
  }
}

export default App;
