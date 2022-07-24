import { parse } from "flags/mod.ts";
import { AlgorithmInput } from "djwt/algorithm.ts";
import { info, warning } from "log/mod.ts";

const CONFIG_FILENAME_ENV = "OAK_CONFIG_FILE";
const CONFIG_FILENAME = Deno.env.get(CONFIG_FILENAME_ENV) || "oak.json";

export interface Config {
  port: number;
  publicKey: string;
  algorithm: AlgorithmInput;
  hostname: string;
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
    info(
      `Loading config file: ${CONFIG_FILENAME}. Note: set the location of the config file using \`${CONFIG_FILENAME_ENV}\``,
    );
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

  info("Configuration loaded succesfully");
  info(config);
  return config;
}
