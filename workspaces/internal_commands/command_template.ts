import { DevKitCommand } from "@devkit/DevKitCommand";

export const Commands = new DevKitCommand({
  name: "<Your Package Name>",
  description: "<Your Package Description>",
  commands: {
    "<your-first-command>": "A description for using your command",
    "<your-second-command>": "A description for using your command",
    "<your-third-command>": "A description for using your command"
  }
})