import type { ICommand, IDevKitConfig } from "./types";

export class DevKitConfig implements Required<IDevKitConfig> {
  project: string;
  commands: Record<string, ICommand>;
  constructor({ project, commands = {} }: IDevKitConfig) {
    this.project = project;
    this.commands = commands;
  }
}
