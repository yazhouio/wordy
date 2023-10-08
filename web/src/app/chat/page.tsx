"use client"

import ChatItem from "@/app/chat/ChatItem";
import {Button, Card, CardBody, Input} from "@nextui-org/react";
import * as React from 'react'

const Chat = () => {
  let data = [
    {
      message: '月亮的英文是`moon`。它是地球的天然卫星，是在夜空中能够看到的亮物体之一。下面是一些关于月亮的英文解释和例句：\n\n解释：The moon is a natural satellite of the Earth that can be seen in the night sky.\n\n例句：\n1. `Look at how bright the moon is tonight.`\n2. `The moon orbits around the Earth.`\n3. `During a full moon, the entire surface of the moon is visible.`',
      isMe: false,
    },
    {
      message: '月亮的英文是`moon`。它是地球的天然卫星，是在夜空中能够看到的亮物体之一。下面是一些关于月亮的英文解释和例句：\n\n解释：The moon is a natural satellite of the Earth that can be seen in the night sky.\n\n例句：\n1. `Look at how bright the moon is tonight.`\n2. `The moon orbits around the Earth.`\n3. `During a full moon, the entire surface of the moon is visible.`',
      isMe: true,
    }
  ]
  return (
      <main className="flex min-h-screen min-w-full p-4 flex-col">
        <h1 className="flex-none">英语启蒙</h1>
        <div className='flex flex-auto flex-col'>
          <div className='flex-auto'>
            {
              data.map(
                  (item, index) => <ChatItem key={index} {...item} />
              )
            }
          </div>
          <Card className='flex-none'>
            <CardBody className='flex flex-row gap-2'>
              <div className='flex-auto'>
                <Input placeholder={'请输入'}/>
              </div>
              <Button className='flex-none'>发送</Button>
            </CardBody>
          </Card>
        </div>
      </main>
  )
}

export default Chat
