import { EventType, WsRequest } from "@/app/interfaces";
import { log } from "console";
import { BehaviorSubject, Observable, Subject, mergeWith } from "rxjs";
import { filter, scan, skip } from 'rxjs/operators';

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

export const hasLoginSuccess$ = new Subject<boolean>();
export const hasLoginSuccess = hasLoginSuccess$.asObservable();

export const login$ = new BehaviorSubject<{
  accessToken: string;
  refreshToken: string;
} | null>(null);
export const logout$ = login$.pipe(
    filter((value) => value === null),
    skip(1)
)
