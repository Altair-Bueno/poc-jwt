import { parse } from "flags/mod.ts";
import { AlgorithmInput } from "djwt/algorithm.ts";
import { info, warning } from "log/mod.ts";
import type { SQLite3Options } from "denodb/mod.ts";

const CONFIG_FILENAME_ENV = "OAK_CONFIG_FILE";
const CONFIG_FILENAME = Deno.env.get(CONFIG_FILENAME_ENV) || "oak.json";

export interface Config {
  port: number;
  publicKey: string;
  algorithm: AlgorithmInput;
  hostname: string;
  database: SQLite3Options;
}

export const defaultConfig = {
  port: 9200,
  algorithm: "RS256",
  hostname: "0.0.0.0",
};

export async function loadConfig(): Promise<Config> {
  const args = parse(Deno.args);

  let file = {};
  try {
    info(`Loading config file: ${CONFIG_FILENAME}.`);
    const content = await Deno.readTextFile(CONFIG_FILENAME);
    file = JSON.parse(content);
  } catch {
    warning(`No configuration file provided. Expected ${CONFIG_FILENAME}`);
  }

  const config = {
    ...defaultConfig,
    ...file,
    ...args,
  } as unknown as Config;

  info("Configuration loaded successfully");
  info(config);
  return config;
}
