import { Router } from "oak/mod.ts";
import { State } from "../types.ts";
import { getDinos} from "../service/dino.ts"

const router = new Router<State>();

router.get("/", (ctx) => {
  ctx.response.body = "Hello world";
});

router.get("/dinos", async (ctx) => {
  const dinos = await getDinos();
  ctx.response.body = dinos;
});

export default router;
