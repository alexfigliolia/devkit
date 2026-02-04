import type { RepoKitCommand } from "./RepoKitCommand";

export interface IRepoKitConfig {
  project: string;
  thirdParty?: RepoKitCommand[];
  commands?: Record<string, ICommand>;
}

export interface IRepoKitCommand {
  name: string;
  owner?: string;
  description: string;
  commands: Record<string, ICommand>;
}

export interface ICommand {
  command: string;
  description: string;
  args?: Record<string, string>;
}

export interface ILocatedCommand extends IRepoKitCommand {
  location: string;
}

export type AsyncTask<T> = () => Promise<T>;
