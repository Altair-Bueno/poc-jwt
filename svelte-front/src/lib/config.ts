import {
  Configuration as AuthServConfiguration,
  DefaultApi as AuthServDefaultApi,
} from "../api/authserv";
import {
  Configuration as SpringPingConfiguration,
  DefaultApi as SpringPingDefaultApi,
} from "../api/spring-ping";
import {
  Credentials,
  credentialStore,
  getExpires,
  hasExpired,
} from "./credentials";
import env from "./env";
import { get } from "svelte/store";

async function refreshCredentials(credentials: Credentials) {
  const conf = new AuthServConfiguration(env.authServConf);
  const client = new AuthServDefaultApi(conf);
  const response = await client.authRefreshPost({
    username: credentials.username,
    refresh_token: credentials.refreshToken,
  });
  const session = response.data;

  credentialStore.update((x) => {
    return {
      accessToken: {
        token: session.access_token,
        expires: getExpires(session.expires_in),
      },
      ...x,
    };
  });
}

async function getToken(): Promise<string> {
  const credentials = get(credentialStore);
  if (!credentials) throw Error("The user has not logged in");

  if (hasExpired(credentials.accessToken.expires)) {
    await refreshCredentials(credentials);
  }
  return credentials.accessToken.token;
}

export const authServClient = new AuthServDefaultApi(
  new AuthServConfiguration({
    accessToken: getToken,
    ...env.authServConf,
  })
);
export const springPingClient = new SpringPingDefaultApi(
  new SpringPingConfiguration({
    accessToken: getToken,
    ...env.springPingConf,
  })
);
