import { Context } from "oak/mod.ts";
import config from "../config.ts";
import { verify } from "djwt/mod.ts";

const AUTHORIZATION_HEADER = "Authorization";
const BEARER_TOKEN_PREFIX = "Bearer";

const key = await Deno.readTextFile(config.publicKey);

function extractToken(rawHeader: string | null | undefined) {
  if (!rawHeader) throw new Error("Missing authorization header");
  if (!rawHeader.startsWith(BEARER_TOKEN_PREFIX)) {
    throw new Error("Invalid authoritzation type. Expected Bearer");
  }
  const token = rawHeader.slice(BEARER_TOKEN_PREFIX.length).trim();
  return token;
}

export async function jwtAuth(ctx: Context, next: any) {
  const authorization = ctx.request.headers.get(AUTHORIZATION_HEADER);
  const token = extractToken(authorization);

  const payload = await verify(token, key, config.algorithm);
  ctx.state.auth = payload;
  await next();
}