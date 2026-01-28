import { DevKitCommand } from "@devkit/DevKitCommand";

export const Commands = new DevKitCommand({
  name: "test",
  description: "<Your Package Description>",
  commands: {
    "install": "yarn install",
  }
})