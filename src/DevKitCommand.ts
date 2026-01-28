import { IDevKitCommand } from "./types";

export class DevKitCommand implements IDevKitCommand {
  name: string;
  description: string;
  commands: Record<string, string>;
  constructor({ name, description, commands = {} }: IDevKitCommand) {
    this.name = name;
    this.commands = commands;
    this.description = description;
  }
}