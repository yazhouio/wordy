export const clientBase64ToString = (base64String: string) => {
  if (typeof base64String !== "string") {
    return "";
  }
  return decodeURIComponent(escape(atob(base64String)));
};

export const clientStringToBase64 = (string: string): string => {
  if (typeof string !== "string") {
    return "";
  }
  return btoa(unescape(encodeURIComponent(string)));
};

export const serverBase64ToString = (base64String: string) => {
  if (typeof base64String !== "string") {
    return "";
  }
  return Buffer.from(base64String, "base64").toString();
};

export const serverStringToBase64 = (string: string) => {
  if (typeof string !== "string") {
    return "";
  }
  return Buffer.from(string).toString("base64");
};
