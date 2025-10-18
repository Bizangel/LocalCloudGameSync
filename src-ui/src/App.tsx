import { useCallback, useState } from 'react'
import './App.css'
import { useWebViewEvent } from './hooks/useGlobalRustEventListener'
import { IPC, type WebViewState } from './ipc/common';
import LoadingDisplay from './LoadingDisplay';
import ErrorDisplay from './ErrorDisplay';
import SuccessDisplay from './SuccessDisplay';
import ConflictDisplay from './ConflictDisplay';
import RemoteEmptyDisplay from './RemoteEmptyDisplay';

function App() {
  const [webViewState, setWebViewState] = useState<WebViewState>("Loading");
  const [display, setDisplay] = useState({ title: "Loading", subtext: "" });

  useWebViewEvent("WebViewStateChange", useCallback((ev) => {
    setWebViewState(ev.state);
  }, [setWebViewState]));

  useWebViewEvent("WebViewUpdate", useCallback((ev) => {
    setDisplay({ title: ev.title_text, subtext: ev.sub_text });
  }, [setDisplay]));

  const sendClose = useCallback(() => {
    IPC.sendUserChoice("close");
  }, [])

  const sendRetrySync = useCallback(() => {
    IPC.sendUserChoice("retry");
  }, [])

  const sendContinueOffline = useCallback(() => {
    IPC.sendUserChoice("continue-offline");
  }, [])

  const sendPush = useCallback(() => {
    IPC.sendUserChoice("push");
  }, [])

  const sendPull = useCallback(() => {
    IPC.sendUserChoice("pull");
  }, [])

  switch (webViewState) {
    case "Loading":
      return <LoadingDisplay {...{display}}/>
    case "Error":
      return <ErrorDisplay error={display}
        onClose={sendClose}
        onRetry={sendRetrySync}
        onContinueOffline={sendContinueOffline}
      />;
    case "Conflict":
      return <ConflictDisplay
        conflict={{
          "localModified": "Thursday, October 21 2021 7:32PM", "remoteModified": "Thursday, October 20 2021 7:00PM"
        }}
        onChooseLocal={sendPush} // keep local -> so push into remote
        onChooseRemote={sendPull} // keep remote -> so pull from remote
      />;
    case "Success":
      return <SuccessDisplay {...{display}} />;
    case "RemoteEmpty":
      return (
        <RemoteEmptyDisplay
          title={display.title}
          subtext={display.subtext}
          onConfirmPush={sendPush}
          onCancel={sendClose}
        />
      )
  }
}

export default App;
