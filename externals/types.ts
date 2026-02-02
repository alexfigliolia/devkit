export interface IRepoKitConfig {
  project: string;
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
}

export interface ILocatedCommand extends IRepoKitCommand {
  location: string;
}

export type AsyncTask<T> = () => Promise<T>;
