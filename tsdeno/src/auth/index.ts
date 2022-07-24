import {Context, Middleware} from "oak/mod.ts";
import {verify} from "djwt/mod.ts";
import {Config} from "../config.ts";
import {State} from "../types.ts";

const AUTHORIZATION_HEADER = "Authorization";
const BEARER_TOKEN_PREFIX = "Bearer";

//const key = await Deno.readTextFile(config.publicKey);

export interface Authentication {
  iss: string;
  sub: string;
  exp: number;
  iat: number;
  roles: Array<string>;
}

function extractToken(rawHeader: string | null | undefined) {
  if (!rawHeader) throw new Error("Missing authorization header");
  if (!rawHeader.startsWith(BEARER_TOKEN_PREFIX)) {
    throw new Error("Invalid authoritzation type. Expected Bearer");
  }
  const token = rawHeader.slice(BEARER_TOKEN_PREFIX.length).trim();
  return token;
}

export function jwtAuth(
  publicKey: string,
  config: Config,
): Middleware<State, Context<State, State>> {
  return async (ctx, next) => {
    const authorization = ctx.request.headers.get(AUTHORIZATION_HEADER);
    const token = extractToken(authorization);

    const payload = await verify(
      token,
      publicKey,
      config.algorithm,
    );
    ctx.state.auth = payload as unknown as Authentication;
    await next();
  };
}
