import React from "react";
import { IWsContext, WsRequest } from "../interfaces";
import { initWs } from "./websocket";
import { message$, localMessage$ } from "./subjects";

export const useInitWs = (accessToken?: string) => {
  const ref = React.useRef(0);
  const [send, setSend] = React.useState<IWsContext>({});

  React.useLayoutEffect(() => {
    console.log(111, accessToken)
    if (!accessToken) {
      return;
    }
    ref.current = 1;
    const { process$, connection$ } = initWs(accessToken);
    let proSubscription = process$.subscribe((message) => {
      message$.next(message);
    });
    setSend({
      send: (message: WsRequest) => {
        localMessage$.next({ ...message, isMe: true });
        connection$.next(message);
      },
    });
    return () => {
      proSubscription.unsubscribe();
      // conSubscription.unsubscribe();
    };
  }, [accessToken]);
  return send;
};
