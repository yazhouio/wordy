import {Subject, timer, pipe, mergeWith, combineLatestWith} from "rxjs";
import {filter, scan,} from 'rxjs/operators';
import {retry, tap} from "rxjs/operators";
import {webSocket, WebSocketSubject} from "rxjs/webSocket";
import {IWsContext, WsRequest} from "@/app/interfaces";
import React from "react";

export const openEvent$ = new Subject<Event>();
export const closeEvent$ = new Subject<CloseEvent>();
export const message$ = new Subject<any>();

export const localMessage$ = new Subject<WsRequest>();
export const chat$ = message$.pipe(
    filter(
        (message: WsRequest) => message.eventType === "chat"
    )
)

export const chatList$ = chat$.pipe(
    mergeWith(localMessage$),
    scan(
        (acc: WsRequest[], currentValue) => {
            const accumulatedData = [...acc, currentValue];
            return accumulatedData;
        }, []),
)

export const WsContext = React.createContext<IWsContext>({});
export const initWs = (uid: string) => {
    const WS_ENDPOINT = process.env.NEXT_PUBLIC_WEBSOCKET_URL || "ws://localhost:3000/ws";


    const connection$: WebSocketSubject<WsRequest> = webSocket({
        url: `${WS_ENDPOINT}?uid=${uid}`,
        openObserver: openEvent$,
        closeObserver: closeEvent$,
        deserializer: (e: MessageEvent) => {
            return JSON.parse(e.data);
        },
        serializer: (value: WsRequest) => {
            // console.log(value)
            return JSON.stringify(value);
        }
    });

    const process$ = connection$.pipe(
        tap({
                error: (error) => console.error(error),
                complete: () => console.log("connection closed"),
            }
        ),
        retry({
            count: 5000,
            resetOnSuccess: true,
            delay: (_, retryCount) => {
                return timer((retryCount + 1) * 5000);
            }
        }),
    );
    return {
        connection$,
        process$,
    }

}
