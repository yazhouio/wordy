'use client'

import { WsContext } from "@/app/lib/websocket";
import { NextUIProvider } from '@nextui-org/react';
import * as React from "react";
import { useInitWs } from './lib/hooks';


export function Providers({children}: { children: React.ReactNode }) {
    const send = useInitWs()
    return (
        <WsContext.Provider value={send}>
            <NextUIProvider>
                {children}
            </NextUIProvider>
        </WsContext.Provider>

    )
}
