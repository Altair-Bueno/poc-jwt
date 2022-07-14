import {
    Configuration as AuthServConfiguration,
    DefaultApi as AuthServDefaultApi
} from "../api/authserv";
import {
    Configuration as SpringPingConfiguration,
    DefaultApi as SpringPingDefaultApi
} from "../api/spring-ping";
import {
    Credentials,
    credentialStore,
    getExpireDate,
    hasExpired
} from "./credentials";
import {get} from "svelte/store";

const config = {
    springPing: {
        basePath: import.meta.env.VITE_SPRINGPING_URL
    },
    authServ: {
        basePath: import.meta.env.VITE_AUTHSERV_URL
    }
}

async function refreshCredentials(credentials: Credentials) {
    const conf = new AuthServConfiguration(config.authServ)
    const client = new AuthServDefaultApi(conf)
    const response = await client.authRefreshPost({
        username: credentials.username,
        refresh_token: credentials.refreshToken
    })
    const session = response.data

    credentialStore.update(x => {
        return {
            accessToken: {
                token: session.access_token,
                expires: getExpireDate(session.expires_in)
            }, ...x
        }
    })
}

async function getToken(): Promise<string> {
    const credentials = get(credentialStore)
    if (!credentials) throw Error("The user has not logged in")

    if (hasExpired(credentials.accessToken.expires)) {
        await refreshCredentials(credentials)
    }
    return credentials.accessToken.token
}

const authServConf = new AuthServConfiguration({
    accessToken: getToken,
    ...config.authServ
})
const springPingConf = new SpringPingConfiguration({
    accessToken: getToken,
    ...config.springPing
})

export const authServClient = new AuthServDefaultApi(authServConf)
export const springPingClient = new SpringPingDefaultApi(springPingConf)