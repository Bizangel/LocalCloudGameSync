
const _IPCRequests = ["webview-ready", "resolve-conflict", "resolve-error"] as const;
type IPCRequest = typeof _IPCRequests[number];

type ResolveConflictType = "pull" | "push";
type ResolveErrorType = "close" | "retry" | "continue-offline";

type WebViewRequest = {
  request: IPCRequest,
  body: any,
}

function _postIPC(msg: WebViewRequest) {
  window.ipc.postMessage(JSON.stringify(msg))
}

export const IPC = {
  sendWebViewReady() {
    _postIPC({ request: "webview-ready", body: ""});
  },

  sendResolveConflict(choice: ResolveConflictType) {
    _postIPC({ request: "resolve-conflict", body: choice});
  },

  sendErrorResolve(choice: ResolveErrorType) {
    _postIPC({ request: "resolve-error", body: choice});
  }
};


/// Types from Rust

export type WebViewUpdateEvent = {
  title_text: string,
  sub_text: string
}

export type WebViewState = "Loading" | "Conflict" | "Error" | "Success" | "RemoteEmpty"

export type WebViewStateChangeEvent = {
  state: WebViewState,
}

export type WebViewEvent = { WebViewUpdate: WebViewUpdateEvent } | { WebViewStateChange : WebViewStateChangeEvent };

export type WebViewEventMap = {
  [E in WebViewEvent as keyof E]: E[keyof E];
};
