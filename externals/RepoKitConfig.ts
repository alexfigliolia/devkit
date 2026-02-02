import type { ICommand, IRepoKitConfig } from "./types";

export class RepoKitConfig implements Required<IRepoKitConfig> {
  project: string;
  commands: Record<string, ICommand>;
  constructor({ project, commands = {} }: IRepoKitConfig) {
    this.project = project;
    this.commands = commands;
  }
}
