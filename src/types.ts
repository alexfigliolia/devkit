export interface IDevKitConfig {
  project: string;
  workspaces: string[];
  commands?: Record<string, string>;
}

export interface IDevKitCommand {
  name: string;
  description: string;
  commands: Record<string, string>;
}

export interface ILocatedCommand extends IDevKitCommand {
  location: string;
}