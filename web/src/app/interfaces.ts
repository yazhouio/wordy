export interface Event {
    loading?: boolean;
    chat?: string;
    speech?: string;
    error?: string;
}

export enum EventType {
    Chat = "chat",
    Speech = "speech",
    Error = "error",
    Loading = "loading",
}
// 结构体 WsRequest
export interface WsRequest {
    from: number;
    to: number;
    event: Event;
    eventType: EventType;
    msgId: string;
    replyMsgId?: string;
    isMe?: boolean;
}

type WsResponse = WsRequest;

export interface IWsContext {
    send?: (message: WsRequest) => void;
}
