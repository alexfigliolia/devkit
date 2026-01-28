import { IDevKitConfig } from "./types";

export class DevKitConfig implements Required<IDevKitConfig> {
  project: string;
  workspaces: string[];
  commands: Record<string, string>;
  constructor({ project, workspaces, commands = {} }: IDevKitConfig) {
    this.project = project;
    this.workspaces = workspaces;
    this.commands = commands;
  }
}