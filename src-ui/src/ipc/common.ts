
const _IPCRequests = ["webview-ready", "resolve-conflict"] as const;
type IPCRequest = typeof _IPCRequests[number];

type ResolveConflictType = "pull" | "push";

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
  }
};


/// Types from Rust

export type WebViewUpdateEvent = {
  display_text: string
}

export type WebViewEvent = | { WebViewUpdate: WebViewUpdateEvent };

