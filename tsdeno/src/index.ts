import { Application } from "oak/mod.ts";
import { loadConfig } from "./config.ts";
import { info } from "log/mod.ts";
import router from "./routes/index.ts";
import { oakCors } from "cors/mod.ts";
import { jwtAuth } from "./auth/index.ts";
import type { State } from "./types.ts";
import {errorMiddleware} from "./error.ts"

const server = new Application<State>();
const config = await loadConfig();
const publicKey = await Deno.readTextFile(config.publicKey);

const baseState: State = {
  config,
  publicKey,
};
// Enable CORS
server.use(errorMiddleware)
server.use(oakCors());
// Dependency injection
server.use(async (ctx, next) => {
  ctx.state = baseState;
  await next();
});
// Require JWT on all routes
server.use(jwtAuth);
server.use(router.routes());
server.listen(config);

info("Server start");
