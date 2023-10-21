"use client";
import "./globals.css";
import { Providers } from "@/app/providers";
import type { Metadata } from "next";
import { Inter } from "next/font/google";
import React from "react";
import { useInitWs } from "./lib/hooks";
import { WsContext } from "./lib/websocket";
import { getCookie } from "cookies-next";
import { closeEvent$, login$ } from "./lib/subjects";
import { Router } from "next/router";
import Toasts from "./login/toasts";
// const argon2 = require('argon2-browser')

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const accessToken = getCookie("access_token");
  const send = useInitWs(accessToken);
  React.useEffect(() => {
    const wsClose = closeEvent$.subscribe(() => {
      Toasts.open({
        title: "连接断开",
        content: "请检查网络",
        type: "error",
      });
    });
    return () => {
      wsClose.unsubscribe();
    };
  }, []);
  return (
    <html lang="en" className="dark">
      <body>
        <Providers>
          <WsContext.Provider value={send}>
            {children}
            <div id="base-modal" />
          </WsContext.Provider>
        </Providers>
      </body>
    </html>
  );
}
