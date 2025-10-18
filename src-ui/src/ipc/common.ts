
const _IPCRequests = ["webview-ready", "user-choice"] as const;
type IPCRequest = typeof _IPCRequests[number];

type UserChoiceType = "pull" | "push" | "close" | "retry" | "continue-offline";;

type WebViewRequest = {
  event_type: IPCRequest,
  body: any,
}

function _postIPC(msg: WebViewRequest) {
  window.ipc.postMessage(JSON.stringify(msg))
}

export const IPC = {
  sendWebViewReady() {
    _postIPC({ event_type: "webview-ready", body: ""});
  },

  sendUserChoice(choice: UserChoiceType) {
    _postIPC({ event_type: "user-choice", body: choice});
  },
};


/// Types from Rust
export type WebViewUpdateCommand = {
  title_text: string,
  sub_text: string
}

export type WebViewState = "Loading" | "Conflict" | "Error" | "Success" | "RemoteEmpty"

export type WebViewStateChangeEvent = {
  state: WebViewState,
}

export type WebViewCommand = { WebViewUpdate: WebViewUpdateCommand } | { WebViewStateChange : WebViewStateChangeEvent };

export type WebViewCommandMap = {
  [E in WebViewCommand as keyof E]: E[keyof E];
};
