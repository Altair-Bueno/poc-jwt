import { parse } from "flags/mod.ts";
import { AlgorithmInput } from "djwt/algorithm.ts";
import { info } from "log/mod.ts"

const CONFIG_FILENAME = 'oak.json'

export interface Config {
  port: number;
  publicKey: string;
  algorithm: AlgorithmInput;
}

const defaultConfig = {
  port: 9200,
  algorithm: "RS256",
};

const args = parse(Deno.args);

let file = {};
try {
  const content = await Deno.readTextFile(CONFIG_FILENAME);
  file = JSON.parse(content)
} catch {
  info(`No configuration file provided. Expected ${CONFIG_FILENAME}`);
}

const config = { 
  ...defaultConfig, 
  ...file,
  ...args 
} as unknown as Config;


info("Configuration loaded succesfully");
info(config);

export default config;
