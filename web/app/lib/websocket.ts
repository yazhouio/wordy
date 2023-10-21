import { IWsContext, WsRequest } from "@/app/interfaces";
import React from "react";
import { timer } from "rxjs";
import { retry } from 'rxjs/operators';
import { WebSocketSubject, webSocket } from "rxjs/webSocket";
import { closeEvent$, openEvent$ } from "./subjects";



export const WsContext = React.createContext<IWsContext>({});

export const initWs = (accessToken: string) => {
  const WS_ENDPOINT = process.env.NEXT_PUBLIC_WEBSOCKET_URL || "ws://localhost:3000/ws";


  const connection$: WebSocketSubject<WsRequest> = webSocket({
    url: `${WS_ENDPOINT}?accessToken=${accessToken}`,
    openObserver: openEvent$,
    closeObserver: closeEvent$,
    deserializer: (e: MessageEvent) => {
      return JSON.parse(e.data);
    },
    serializer: (value: WsRequest) => {
      return JSON.stringify(value);
    }
  });

  const process$ = connection$.pipe(
      retry({
        count: 1000,
        resetOnSuccess: true,
        delay: (_, retryCount) => {
          console.log(`retry attempt: ${retryCount + 1}`);
          return timer((retryCount + 1) * 5000);
        }
      }),
  );
  return {
    connection$,
    process$,
  }

}
