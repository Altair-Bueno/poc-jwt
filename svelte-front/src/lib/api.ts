import config from "./config";

const {endpoints} = config

export type Credentials = {
    username: string,
    password: string
}

export interface JWTSession {
    access_token: string,
    refresh_token: string,
    token_type: string,
    expires_in: string,
}

export async function checkAuth(session: JWTSession) {
    const url = `${endpoints.authServ}/auth/check`
    const request = {
        method: "GET",
        headers: {
            "Authorization": `${session.token_type} ${session.access_token}`
        }
    }
    const response = await fetch(url, request)

    console.log([url, request, response])

    if (response.ok) {
        // nothing
    } else {
        throw Error("Unexpected response")
    }
}

export async function register(credentials: Credentials) {
    const url = `${endpoints.authServ}/auth/register`
    const request = {
        body: JSON.stringify(credentials),
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        }
    }
    const response = await fetch(url, request)

    console.log([url, request, response])

    if (response.ok) {
        // nothing
    } else {
        throw Error("Unexpected response")
    }
}

export async function refresh(session: JWTSession) {
    const url = `${endpoints.authServ}/auth/refresh`
    const request = {
        body: JSON.stringify({"refresh_token": session.refresh_token}),
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        }
    }
    const response = await fetch(url, request)
    const json = await response.json()

    console.log([url, request, response, json])

    if (response.ok) {
        return json as JWTSession
    } else {
        throw Error("Unexpected response")
    }
}

export async function login(credentials: Credentials) {
    const url = `${endpoints.authServ}/auth/login`
    const request = {
        body: JSON.stringify(credentials),
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        }
    }
    const response = await fetch(url, request)
    const json = await response.json()

    console.log([url, request, response, json])

    if (response.ok) {
        return json as JWTSession
    } else {
        throw Error("Unexpected response")
    }
}

export async function springPing(session: JWTSession, content: string) {
    const url = `${endpoints.springPing}/`
    const request = {
        method: "POST",
        body: content,
        headers: {
            "Content-Type": "text/plain",
            "Authorization": `${session.token_type} ${session.access_token}`
        }
    }
    const response = await fetch(url, request)

    console.log([url, request, response])

    if (response.ok) {
        return await response.text()
    } else {
        throw Error("Unexpected response")
    }
}