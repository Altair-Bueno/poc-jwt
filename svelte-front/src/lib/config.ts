import {
  Configuration as AuthServConfiguration,
  DefaultApi as AuthServDefaultApi,
} from "../api/authserv";
import {
  Configuration as SpringPingConfiguration,
  DefaultApi as SpringPingDefaultApi,
} from "../api/spring-ping";
import {
  Configuration as AxumTransformConfiguration,
  DefaultApi as AxumTransformDefaultApi,
} from "../api/axum-transform";
import {
  Credentials,
  credentialStore,
  getExpires,
  hasExpired,
} from "./credentials";
import env from "./env";
import { get } from "svelte/store";

async function refreshCredentials(
  credentials: Credentials
): Promise<Credentials> {
  const conf = new AuthServConfiguration(env.authServConf);
  const client = new AuthServDefaultApi(conf);

  const { refreshToken, username } = credentials;
  const response = await client.authRefreshPost({
    username,
    refresh_token: refreshToken,
  });
  const session = response.data;

  const expires = getExpires(session.expires_in - env.credentialsConf.margin);
  const token = session.access_token;
  const accessToken = { token, expires };

  return { refreshToken, username, accessToken };
}

async function getToken(): Promise<string> {
  let credentials = get(credentialStore);
  if (!credentials) throw Error("The user has not logged in");

  if (hasExpired(credentials.accessToken.expires)) {
    credentials = await refreshCredentials(credentials);
    credentialStore.set(credentials);
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

export const axumTransformClient = new AxumTransformDefaultApi(
  new AxumTransformConfiguration({
    accessToken: getToken,
    ...env.axumTransformConf,
  })
);
