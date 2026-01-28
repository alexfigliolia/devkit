use std::collections::HashMap;

use crate::{
    configuration::configuration::DevKitCommand, executables::executable::Executable,
    logger::logger::Logger,
};

pub struct Help;

impl Help {
    pub fn internal_command(command: Executable) {
        println!(
            "{}",
            format!(
                "{}{}",
                Logger::indent(Some(5)),
                Logger::blue_bright(&command.name)
            )
        );
        Help::print_args(command.args);
    }

    pub fn external_command(command: &DevKitCommand) {
        println!("{}", Logger::blue_bright(&command.name));
        Help::print_args(command.commands.clone());
    }

    fn print_args(map: HashMap<String, String>) {
        for (name, description) in map {
            println!(
                "{}",
                format!(
                    "{}{}{}",
                    Logger::indent(Some(3)),
                    Logger::magenta_bright(format!("{}: ", name).as_str()),
                    description,
                )
            );
        }
    }
}
