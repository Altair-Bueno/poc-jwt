import { Router } from "oak/mod.ts";
import { State } from "../types.ts";
import { getDinos, newDino } from "../service/dino.ts";

const router = new Router<State>();

router.get("/", (ctx) => {
  ctx.response.body = "Hello world";
});

router.get("/dinos", async (ctx) => {
  const dinos = await getDinos();
  ctx.response.body = dinos;
});

router.post("/dinos", async (ctx) => {
  const payload = await ctx.request.body({ type: "json" }).value;
  await newDino(payload);
});

export default router;
