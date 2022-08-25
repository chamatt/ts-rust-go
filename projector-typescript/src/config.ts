import envPaths from "env-paths";
import path from "path";
import { Opts } from "./ops";

const paths = envPaths("projector");
const CONFIG_PATH = paths.config;

enum Operation {
  Print,
  Add,
  Remove,
}

export type Config = {
  args: string[];
  operation: Operation;
  config: string;
  pwd: string;
};

function getPwd(opts: Opts): string {
  if (opts.pwd) {
    return opts.pwd;
  }
  return process.cwd();
}

function getConfigPath(opts: Opts): string {
  if (opts.config) {
    return opts.config;
  }

  return path.join(CONFIG_PATH, ".projector.json");
}

function getOperation(opts: Opts): Operation {
  if (!opts.args || opts.args.length === 0) {
    return Operation.Print;
  }
  if (opts.args[0] === "add") {
    return Operation.Add;
  }
  if (opts.args[0] === "remove") {
    return Operation.Remove;
  }
  throw new Error(`Unknown operation: ${opts.args[0]}`);
}

function getArgs(opts: Opts): string[] {
  if (!opts.args || opts.args.length === 0) {
    return [];
  }
  const operation = getOperation(opts);
  switch (operation) {
    case Operation.Print: {
      if (opts.args.length > 1) {
        throw new Error(
          `Expected 0 or 1 arguments for operation \`Print\` but got ${opts.args.length}`
        );
      }
    }

    case Operation.Add: {
      if (opts.args.length !== 3) {
        throw new Error(
          `Expected 2 arguments for operation \`Add\` but got ${
            opts.args.length - 1
          }`
        );
      }
      return opts.args.slice(1);
    }

    case Operation.Remove: {
      if (opts.args.length !== 2) {
        throw new Error(
          `Expected 1 arguments for operation \`Remove\` but got ${
            opts.args.length - 1
          }`
        );
      }
      return opts.args.slice(1);
    }
  }
}

export default function config(opts: Opts): Config {
  return {
    pwd: getPwd(opts),
    config: getConfigPath(opts),
    args: getArgs(opts),
    operation: getOperation(opts),
  };
}
