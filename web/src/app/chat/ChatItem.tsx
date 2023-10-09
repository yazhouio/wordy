"use client"
import {PlayIcon} from "@/app/chat/PlayIcon";
import {StopIcon} from "@/app/chat/StopIcon";
import {Accordion, AccordionItem, Avatar, Button, Card, CardBody, Input} from "@nextui-org/react";
import clsx from "clsx";
import * as React from 'react'
import './styles.css'
interface ChatItemProps {
  isMe?: boolean,
  message?: string,
}

const ChatItem = (props: ChatItemProps) => {
  const {isMe, message = ''} = props
  const lines = message.split('\n').map((line, index) => {
    return <div className='indent-8 text-medium' key={index}>{line}</div>
  })
  const renderMore = () => {
    const list = message.match(/`\w(.*)`/g)?.map(
        text => {
          return <Button startContent={<PlayIcon size={20} className={'flex-none'} />} variant="flat" key={text}><div className='truncate flex-auto text-left'>{text.slice(1, -1)}</div></Button>
        }
    )
    return <div className={'grid gap-3 chat-play-button'}>
      <Button isLoading startContent={<StopIcon size={20} />}><div className='flex-auto text-left'>全文朗读</div></Button>
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
        : <Card  className='w-4/5 '>
          <CardBody>
            {lines}
          </CardBody>
        </Card>}
  </div>
}

export default ChatItem
