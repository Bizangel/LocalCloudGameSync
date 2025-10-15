import { useEffect } from "react"
import { IPC, type WebViewEvent } from "../ipc/common"

export const useWebViewEvent = <K extends keyof WebViewEvent>(key: K, callback: (ev: WebViewEvent[K]) => void) => {
    useEffect(() => {
        const listener = (event: MessageEvent<any>) => {
            if (key in event.data) {
                callback(event.data[key])
            }
        }

        window.addEventListener("message", listener);
        IPC.sendWebViewReady();
        return () => {
            window.removeEventListener("message", listener);
        }
    }, [callback, key])
}