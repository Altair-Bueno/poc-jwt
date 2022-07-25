import { Application } from "oak/mod.ts";
import { loadConfig } from "./config.ts";
import { info } from "log/mod.ts";
import router from "./routes/index.ts";
import { oakCors } from "cors/mod.ts";
import { jwtAuth } from "./auth/index.ts";
import type { State } from "./types.ts";
import { errorMiddleware } from "./error.ts";
import { Database, SQLite3Connector } from "denodb/mod.ts";
import { linkModel } from "./database/index.ts";

const config = await loadConfig();
const publicKey = await Deno.readTextFile(config.publicKey);

const connector = new SQLite3Connector(config.database);
const database = new Database(connector);
linkModel(database);

const server = new Application<State>({ state: { config } });
server.use(errorMiddleware);
// Enable CORS
server.use(oakCors());
// Require JWT on all routes
server.use(jwtAuth(publicKey, config));
server.use(router.routes());
server.listen(config);

info("Server start");
