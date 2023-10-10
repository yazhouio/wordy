import {EventType, IWsContext, WsRequest} from "@/app/interfaces";
import React from "react";
import {mergeWith, Observable, Subject, timer} from "rxjs";
import {filter, retry, scan,} from 'rxjs/operators';
import {webSocket, WebSocketSubject} from "rxjs/webSocket";

export const openEvent$ = new Subject<Event>();
export const closeEvent$ = new Subject<CloseEvent>();
export const message$ = new Subject<WsRequest>();

export const localMessage$ = new Subject<WsRequest>();
export const chat$ = message$

export const speech$: Observable<Record<string, string>> = message$.pipe(
    filter(
        (message: WsRequest) => message.eventType === EventType.Speech
    ),
    scan(
        (acc: Record<string, string>, currentValue) => {
          return {...acc, [currentValue.replyMsgId!]: currentValue.event.speech!};
        }, {} as Record<string, string>),
)

export const chatList$ = message$.pipe(
    mergeWith(localMessage$),
    filter(
        (message: WsRequest) => message.eventType === "chat"
    ),
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
      retry({
        count: 5000,
        resetOnSuccess: true,
        delay: (_, retryCount) => {
          return timer((retryCount + 1) * 1000);
        }
      }),
  );
  return {
    connection$,
    process$,
  }

}
