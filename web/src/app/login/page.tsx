import {Password} from "@nextui-org/shared-icons";
import * as React from 'react'
import {Button, Card, CardBody, CardHeader, Input} from '@nextui-org/react'

export default function Login() {
    return (
        <main className={'w-full h-full grid items-center justify-center'}>
        <Card>
            <CardHeader>
                Login
            </CardHeader>
            <CardBody>
              <Input label={'账户'} placeholder="请输入" />
                <Password label={'密码'} />
              <Button onClick={e => {
                    console.log(e)
              }}>登录</Button>
            </CardBody>
        </Card>
        </main>
    )
}
