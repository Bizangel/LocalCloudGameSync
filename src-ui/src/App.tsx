import { useCallback, useState } from 'react'
import './App.css'
import { useWebViewEvent } from './hooks/useGlobalRustEventListener'
import { IPC, type WebViewState, type WebViewUpdateCommand } from './ipc/common';
import LoadingDisplay from './LoadingDisplay';
import ErrorDisplay from './ErrorDisplay';
import SuccessDisplay from './SuccessDisplay';
import ConflictDisplay from './ConflictDisplay';
import RemoteEmptyDisplay from './RemoteEmptyDisplay';

type DisplayType = WebViewUpdateCommand

function App() {
  const [webViewState, setWebViewState] = useState<WebViewState>("Loading");
  const [display, setDisplay] = useState<DisplayType>({ title_text: "Loading", sub_text: "", conflict_info: undefined, is_after_game: false });

  useWebViewEvent("WebViewStateChange", useCallback((ev) => {
    setWebViewState(ev.state);
  }, [setWebViewState]));

  useWebViewEvent("WebViewUpdate", useCallback((ev) => {
    setDisplay(ev);
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
      return <ErrorDisplay
        error={display}
        onClose={sendClose}
        onRetry={sendRetrySync}
        onContinueOffline={sendContinueOffline}
      />;
    case "Conflict":
      if (display.conflict_info === undefined)
        return null;
      return <ConflictDisplay
        title={display.title_text}
        is_after_game={display.is_after_game}
        conflict_info={display.conflict_info}
        onChooseLocal={sendPush} // keep local -> so push into remote
        onChooseRemote={sendPull} // keep remote -> so pull from remote
      />;
    case "Success":
      return <SuccessDisplay {...{display}} />;
    case "RemoteEmpty":
      return (
        <RemoteEmptyDisplay
          title={display.title_text}
          subtext={display.sub_text}
          onConfirmPush={sendPush}
          onCancel={sendClose}
        />
      )
  }
}

export default App;
