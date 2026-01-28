use std::{collections::HashMap, env::args};

use crate::{
    configuration::configuration::{DevKitCommand, DevKitConfig},
    executables::intenal_executable::InternalExecutable,
    executor::executor::Executor,
    external_commands::external_commands::ExternalCommands,
    internal_commands::{help::Help, register_command::RegisterCommand},
    logger::logger::Logger,
};

pub struct DevKit {
    root: String,
    configuration: DevKitConfig,
}

impl DevKit {
    pub fn new(root: String, configuration: DevKitConfig) -> DevKit {
        DevKit {
            root,
            configuration,
        }
    }

    pub fn invoke(&self) {
        let (command, args) = DevKit::parse();
        let internals = self.internal_commands();
        if internals.contains_key(&command) {
            match internals.get(&command) {
                Some(command) => {
                    return command.run(args);
                }
                None => {}
            }
        }
        let externals = self.external_commands();
        ExternalCommands::validate(&internals, &externals);
        if externals.contains_key(&command) {
            if &args.len() <= &0 {
                return Help::external_command(externals.get(&command).expect("found"));
            }
            let sub_command = &args[0];
            match externals.get(&command) {
                Some(devkit) => {
                    if devkit.commands.contains_key(sub_command) {
                        match devkit.commands.get(sub_command) {
                            Some(script) => {
                                let command_args = &args[1..].iter().collect::<Vec<_>>()[..];
                                return Executor::with_stdio(script, Some(command_args));
                            }
                            None => {}
                        }
                    }
                }
                None => {}
            }
            return self.subcommand_not_found(&command, &sub_command);
        }
        return self.command_not_found(&command);
    }

    fn parse() -> (String, Vec<String>) {
        let argv: Vec<String> = args().collect();
        let command = &argv[1];
        let args = &(&argv)[2..];
        return (command.clone(), args.to_vec());
    }

    fn internal_commands(&self) -> HashMap<String, RegisterCommand> {
        let commands = [RegisterCommand::new(self.root.clone())];
        return HashMap::from(commands.map(|x| (x.definition.name.clone(), x)));
    }

    fn external_commands(&self) -> HashMap<String, DevKitCommand> {
        let finder = ExternalCommands::new(self.root.clone());
        return futures::executor::block_on(finder.find_all());
    }

    fn command_not_found(&self, command: &str) {
        Logger::info(
            format!(
                "I'm not aware of a command named {}",
                Logger::blue_bright(&command)
            )
            .as_str(),
        );
        Logger::exitWithInfo("Here are the commands I'm aware of");
    }

    fn subcommand_not_found(&self, command: &str, sub_command: &str) {
        Logger::info(
            format!(
                "The command {} was not found on {}",
                Logger::blue_bright(sub_command),
                Logger::blue_bright(command)
            )
            .as_str(),
        );
        Logger::exitWithInfo(
            format!(
                "Here are the commands that belong to {}",
                Logger::blue_bright(sub_command)
            )
            .as_str(),
        );
    }
}
