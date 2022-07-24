import { Authentication } from "./auth/index.ts";
import { Config } from './config.ts'
export interface State {
  config: Config,
  publicKey?: string,
  auth?: Authentication;
}
