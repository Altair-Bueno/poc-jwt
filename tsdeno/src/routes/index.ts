import { Router } from "oak/mod.ts";

const router = new Router();

router.get("/", (ctx) => {
  ctx.response.body = "Hello world";
});

export default router;
