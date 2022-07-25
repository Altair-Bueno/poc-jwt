import { Database, DataTypes, Model } from "denodb/mod.ts";

export class Dino extends Model {
  static table = "dino";
  static timestamps = true;

  static fields = {
    id: { primaryKey: true, autoIncrement: true },
    name: DataTypes.STRING,
  };
}

export function linkModel(db: Database) {
  db.link([Dino]);
  db.sync();
}
