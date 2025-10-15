import { useEffect } from "react"
import { IPC } from "../ipc/common"

export const useGlobalRustEventListener = () => {
    useEffect(() => {
        const listener = (event: MessageEvent<any>) => {
            console.log("received new event!: ", event)
        }

        window.addEventListener("message", listener);
        IPC.sendWebViewReady();
        return () => {
            window.removeEventListener("message", listener);
        }
    }, [])
}