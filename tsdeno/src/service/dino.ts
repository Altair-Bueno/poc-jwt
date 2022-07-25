import { Dino } from "../database/index.ts";

export async function getDinos(page = 0): Promise<Dino[]> {
  const dinos = await Dino.offset(page * 10)
    .limit(10)
    .get();
  return dinos as unknown as Dino[];
}

export async function newDino({ name }: { name: string }): Promise<void> {
  const dino = new Dino();
  dino.name = name;
  await dino.save();
}
