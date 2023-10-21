"use client";

import { Button, Card, CardBody, CardHeader, Input } from "@nextui-org/react";
import { Password, User } from "@nextui-org/shared-icons";
import { useRouter, useSearchParams } from "next/navigation";
import React from "react";
import { useForm } from "react-hook-form";
import Totast from "./toasts";
import { setToken } from "../lib/jwtToken";
import { Router } from "next/router";
const argon2 = require("argon2-browser/dist/argon2-bundled.min.js");

interface Value {
  username: string;
  password: string;
}

export default function Login() {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<Value>();

  const router = useRouter();
  const searchParams = useSearchParams();
  const [salt, setSalt] = React.useState<string>("");
  const onSubmit = async (data: Value) => {
    let salt1 = salt;
    if (!salt1) {
      salt1 = await getSalt(data.username);
      setSalt(salt1);
    }
    const pwd = await argon2.hash({
      pass: data.password,
      salt: salt1,
      hashLen: 32,
      type: argon2.ArgonType.Argon2id,
    });

    fetch("/api/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        name: data.username,
        password: pwd.encoded,
      }),
    })
      .then((res) => {
        console.log(res);
        if (res.status === 200) {
          res.json().then((data) => {
            const referer = searchParams.get('referer') || '/';
            setToken(data);
            router.push(referer);
          });
        } else {
          Totast.open({
            title: "登录失败",
            content: "用户名或密码错误",
            type: "error",
          });
        }
      })
      .catch((err) => {
        Totast.open({
          title: "登录失败",
          content: "网络错误",
          type: "error",
        });
      });
  };

  const getSalt = async (name: string) => {
    const res = await fetch(`/api/${name}/salt`);
    const data = await res.json();
    return data.salt;
  }

  const onBlur: React.FocusEventHandler<any> = (e) => {
    const name = e.target.value;
    console.log(name);
    if (!name) {
      return;
    }
    fetch(`/api/${name}/salt`)
      .then((e) => e.json())
      .then((res) => {
        setSalt(res.salt);
      });
  };

  const userNameProps = register("username", { required: true });
  return (
    <main className={"w-screen h-screen grid items-center justify-center"}>
      <Card className={"w-[600px]"}>
        <CardHeader>Login</CardHeader>
        <CardBody>
          <form
            className={"flex gap-4 flex-col"}
            onSubmit={handleSubmit(onSubmit)}
          >
            <Input
              isRequired
              isInvalid={!!errors.username?.type}
              errorMessage={errors?.username?.type && "Username is required"}
              startContent={
                <User
                  size={16}
                  fill={"white"}
                  className={
                    "text-2xl text-default-400 pointer-events-none flex-shrink-0"
                  }
                />
              }
              label={"账户"}
              placeholder="请输入"
              {...userNameProps}
              onBlur={(e) => {
                onBlur(e);
                userNameProps.onBlur(e);
              }}
            />
            <Input
              isRequired
              isInvalid={!!errors.password?.type}
              startContent={
                <Password
                  size={16}
                  fill={"white"}
                  className={
                    "text-2xl text-default-400 pointer-events-none flex-shrink-0"
                  }
                />
              }
              label={"密码"}
              type={"password"}
              placeholder="请输入"
              {...register("password", { required: true })}
            />
            <Button
              type={"submit"}
              onClick={(e) => {
                console.log(e);
              }}
            >
              登录
            </Button>
          </form>
        </CardBody>
      </Card>
    </main>
  );
}
