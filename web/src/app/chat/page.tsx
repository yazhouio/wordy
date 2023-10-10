"use client"

import ChatItem from "@/app/chat/ChatItem";
import {Button, Card, CardBody, Input} from "@nextui-org/react";
import * as React from 'react'
import {chat$, chatList$, WsContext} from "@/app/websocket";
import {EventType, WsRequest} from "@/app/interfaces";

const Chat = () => {
  const {send} = React.useContext(WsContext);
  const [data, setData] = React.useState<WsRequest[]>([])
  const [value, setValue] = React.useState('')
  React.useEffect(
      () => {
        const sub = chatList$.subscribe(
            (list) => {
              console.log(list)
              setData(list)
            }
        );
        return () => {
          sub.unsubscribe();
        }
      }, []
  )


  return (
      <main className="flex overflow-hidden h-screen min-w-full p-4 flex-col">
        <h1 className="flex-none text-foreground">英语启蒙</h1>
        <div className='flex flex-auto overflow-hidden flex-col'>
          <div className='flex-auto overflow-auto'>
            {
              data.map(
                  (item, index) => <ChatItem key={index}
                                             isMe={item.isMe}
                                             message={item.event?.chat!}
                                             msgId={item.msgId}
                  />
              )
            }
          </div>
          <Card className='flex-none'>
            <CardBody className='flex flex-row gap-2'>
              <div className='flex-auto'>
                <Input placeholder={'请输入'} value={value}
                       onChange={
                         e => {
                           setValue(e.target.value)
                         }
                       }
                />
              </div>
              <Button className='flex-none'
                      onClick={() => {
                        send?.({
                          from: 1,
                          event: {chat: value},
                          eventType: EventType.Chat,
                          to: 0,
                          msgId: 'adad'
                        })
                      }}
              >发送</Button>
            </CardBody>
          </Card>
        </div>
      </main>
  )
}

export default Chat
