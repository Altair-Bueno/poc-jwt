import {writable} from 'svelte/store';

export interface Credentials {
    accessToken: {
        token: string,
        expires: Date
    },
    refreshToken: string,
    username: string,
}

export function getExpireDate(seconds:number) {
    return new Date(new Date().getTime() + seconds * 1000)
}
export function hasExpired(date:Date) {
    const currentTime = new Date().getTime()
    const expireTime = date.getTime()
    return currentTime >= expireTime
}

export const credentialStore = writable(null as null | Credentials)



