import { writable } from "svelte/store";
import env from "./env";

export interface Credentials {
  accessToken: {
    token: string;
    expires: number;
  };
  refreshToken: string;
  username: string;
}

const key = env.credentialsConf.key;
const storedString = localStorage.getItem(key);
const initialCredentials = storedString ? JSON.parse(storedString) : null;

export const credentialStore = writable(
  initialCredentials as null | Credentials
);

credentialStore.subscribe((credentials) => {
  const s = credentials ? JSON.stringify(credentials) : null;
  localStorage.setItem(key, s);
});

export function getExpires(seconds: number) {
  return new Date().getTime() + seconds * 1000;
}
export function hasExpired(expires: number) {
  const currentTime = new Date().getTime();
  return currentTime >= expires;
}
