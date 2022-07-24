import { Application } from "oak/mod.ts";
import config from "./config.ts";
import { info } from "log/mod.ts";
import router from "./routes/index.ts";
import { oakCors } from "cors/mod.ts";
import { jwtAuth } from "./auth/index.ts";

const server = new Application();
server.use(oakCors());
server.use(jwtAuth);
server.use(router.routes());
server.listen(config);

info("Server start");
