"use client"
import {PlayIcon} from "@/app/chat/PlayIcon";
import {StopIcon} from "@/app/chat/StopIcon";
import {EventType, WsRequest} from "@/app/interfaces";
import {message$, WsContext} from "@/app/websocket";
import {Accordion, AccordionItem, Avatar, Button, Card, CardBody} from "@nextui-org/react";
import clsx from "clsx";
import * as React from 'react'
import './styles.css'
import {BehaviorSubject, EMPTY, empty, switchMap} from "rxjs";
import {filter, tap} from "rxjs/operators";

interface ChatItemProps {
  isMe?: boolean,
  message: string,
  msgId?: string,
}

const ActionButton = (props) => {
  const {label, text, msgId} = props
  const {send} = React.useContext(WsContext);
  const [loading$] = React.useState(new BehaviorSubject(false))
  const [isLoading, setIsLoading] = React.useState(loading$.getValue())
  const [isPlaying, setIsPlaying] = React.useState(false)

  const handlePlay = (path?: string) => {
    setIsPlaying(true)
    setIsLoading(false)
    console.log(process.env)
    const audio = new Audio(process.env.NEXT_PUBLIC_ENDPOINT! + path)
    audio.play().then(
        () => {
          setIsPlaying(false)
        }
    ).catch(
        () => {
          setIsPlaying(false)
        }
    )
  }

  React.useEffect(
      () => {
        let sub = loading$.pipe(
            tap((loading) => setIsLoading(loading)),
            switchMap((loading) => {
                  if (!loading) {
                    return EMPTY;
                  }
                  return message$.pipe(
                      filter(
                          (message: WsRequest) => {
                            return message.eventType === EventType.Speech && message.replyMsgId === msgId
                          }
                      ),
                      tap(
                          (message) => {
                            handlePlay(message.event.speech)
                          }
                      )
                  )
                }
            )
        ).subscribe()
        return () => {
          sub.unsubscribe()
        }
      }, []
  )
  const handleClick = () => {
    loading$.next(true)
    send?.({
      from: 1,
      event: {speech: text},
      eventType: EventType.Speech,
      to: 0,
      msgId,
    })
  }

  React.useEffect(
      () => {

      }, []
  )

  const Cmp = isPlaying ? StopIcon : PlayIcon

  return <Button onClick={handleClick} isLoading={isLoading} startContent={<Cmp className='flex-none' size={20}/>}>
    <div className='flex-auto text-left truncate'>{label}</div>
  </Button>
}

const ChatItem = (props: ChatItemProps) => {
  const {isMe, message = '', msgId} = props
  const lines = message.split('\n').map((line, index) => {
    return <div className='indent-8 text-medium' key={index}>{line}</div>
  })
  const renderMore = () => {
    const list = message.match(/`\w.*?`/g)?.map(
        (text, index) => {
          return <ActionButton label={text.slice(1, -1)} text={text.slice(1, -1)} key={text}
                               msgId={msgId + '-' + index}/>
        }
    )
    return <div className={'grid gap-3 chat-play-button'}>
      <ActionButton label={'全文朗读'} text={message} msgId={msgId}/>
      {list}</div>
  }
  return <div className={clsx('flex gap-1 mb-8', {
    'flex-row-reverse': isMe,
    'flex-row': !isMe,
  })}>
    <Avatar className='flex-none mt-2' showFallback/>
    {!isMe ? <Accordion className='w-4/5' variant="splitted">
          <AccordionItem key="1" aria-label="Accordion 1" title={lines}>
            {renderMore()}
          </AccordionItem>
        </Accordion>
        : <Card className='w-4/5 '>
          <CardBody>
            {lines}
          </CardBody>
        </Card>}
  </div>
}

export default ChatItem
