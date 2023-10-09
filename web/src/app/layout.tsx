"use client"
import './globals.css'
import {Providers} from "@/app/providers";
import type {Metadata} from 'next'
import {Inter} from 'next/font/google'
import React from "react";

const inter = Inter({subsets: ['latin']})


export default function RootLayout({
                                     children,
                                   }: {
  children: React.ReactNode
}) {
  return (
      <html lang="en" className='dark'>
      <body>
      <Providers>
        {children}
      </Providers>
      </body>
      </html>
  )
}
