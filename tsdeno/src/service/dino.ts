import { Dino } from "../database/index.ts";

export async function getDinos(page = 0): Promise<Dino[]> {
  const dinos = await Dino.offset(page * 10)
    .limit(10)
    .get();
  return dinos as unknown as Dino[];
}