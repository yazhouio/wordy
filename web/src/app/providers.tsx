'use client'

import {NextUIProvider} from '@nextui-org/react'
import * as React from "react";
import {initWs, localMessage$, message$, openEvent$, WsContext} from "@/app/websocket";
import {IWsContext, WsRequest} from "@/app/interfaces";


export function Providers({children}: { children: React.ReactNode }) {
    const ref = React.useRef(0)
    const [send, setSend] = React.useState<IWsContext>({});
    React.useLayoutEffect(() => {
        if (!ref.current) {
            ref.current = 1
            const {connection$} = initWs('1');
            connection$.subscribe(
                (message) => {
                    message$.next(message);
                }
            )
            setSend(
                {
                    send: (message: WsRequest) => {
                        localMessage$.next({...message, isMe: true});
                        connection$.next(message);
                    }
                }
            )
            return () => {
                // console.log(3333)
                // connection$.unsubscribe();
                // connection$.complete();
            }

        }


    }, [])

    return (
        <WsContext.Provider value={send}>
            <NextUIProvider>

                {children}
            </NextUIProvider>

        </WsContext.Provider>

    )
}
