import { useEffect } from "react"

export const useGlobalRustEventListener = () => {
    useEffect(() => {
        const listener = (event: MessageEvent<any>) => {
            console.log("received new event!: ", event)
        }

        window.addEventListener("message", listener);
        return () => {
            window.removeEventListener("message", listener);
        }
    }, [])
}