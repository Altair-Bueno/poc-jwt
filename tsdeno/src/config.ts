import { AlgorithmInput } from "djwt/algorithm.ts";
import { info } from "log/mod.ts";
import type { SQLite3Options } from "denodb/mod.ts";
import { auto } from "chimera/mod.ts";

const CONFIG_DIR_ENV = "OAK_META_DIR";

export interface Config {
  meta: { dir: string };
  port: number;
  publicKey: string;
  algorithm: AlgorithmInput;
  hostname: string;
  database: SQLite3Options;
}

export async function loadConfig(): Promise<Config> {
  const configDir = Deno.env.get(CONFIG_DIR_ENV) || Deno.cwd();
  info(`Loading configuration from ${configDir}`);

  const config = await auto<Config>({
    name: "oak",
    baseConfig: {
      meta: { dir: configDir },
      port: 9200,
      algorithm: "RS256",
      hostname: "0.0.0.0",
      publicKey: "",
      database: {
        filepath: "",
      },
    },
    configDir,
  });

  info("Configuration loaded successfully");
  info(config);
  return config;
}
