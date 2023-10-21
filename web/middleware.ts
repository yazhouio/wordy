import { deleteCookie, setCookie } from "cookies-next";
import type { NextRequest } from "next/server";
import { NextResponse } from "next/server";
import { serverBase64ToString } from "./app/lib/base64";

function isAuth(req: NextRequest) {
  const cookieStore = req.cookies;
  const access_token = cookieStore.get("access_token");
  const refresh_token = cookieStore.get("refresh_token");
  if (!access_token || !refresh_token) {
    return false;
  }
  try {
    const accessTokenData = JSON.parse(
      serverBase64ToString(access_token.value.split(".")[1])
    );
    const refreshTokenData = JSON.parse(
      serverBase64ToString(refresh_token.value.split(".")[1])
    );
    if (
      accessTokenData.exp < Date.now() / 1000 &&
      refreshTokenData.exp < Date.now() / 1000
    ) {
      return false;
    }
    return true;
  } catch (error) {
    console.error(error);
    return false;
  }
}

export function middleware(req: NextRequest) {
  if (!isAuth(req)) {
    deleteCookie("refresh_token", {path: "/"})
    setCookie("access_token", "", {path: "/"})
    return NextResponse.redirect(
      new URL(`/login?referer=${req.nextUrl.pathname}`, req.url)
    );
  }
  return NextResponse.next();
}

export const config = {
  matcher:
    "/((?!login|ws|api|_next|_next/static|favicon.ico|__nextjs_original-stack-frame|node_modules).*)",
};
