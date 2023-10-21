import { setCookie } from "cookies-next";
import { clientBase64ToString, clientStringToBase64 } from "./base64";

export const setToken = ({ access_token, refresh_token }: {
    access_token: string;
    refresh_token: string;
}) => {
  try {
    const accessTokenData = JSON.parse(clientBase64ToString(access_token.split(".")[1]));
    const refreshTokenData = JSON.parse(clientBase64ToString(refresh_token.split(".")[1]));

    setCookie("access_token", access_token, {
      maxAge: accessTokenData.exp - Date.now() / 1000,
      path: "/",
    });
    setCookie("refresh_token", refresh_token, {
      maxAge: refreshTokenData.exp - Date.now() / 1000,
      path: "/",
    });
  } catch (error) {
    console.error(error);
  }
};

