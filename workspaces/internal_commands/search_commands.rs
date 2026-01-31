use alphanumeric_sort::sort_slice_by_str_key;
use std::collections::HashMap;

use crate::{
    devkit::interfaces::{Command, DevKitCommand, DevKitConfig},
    executables::{
        intenal_executable::InternalExecutable,
        internal_executable_definition::InternalExecutableDefinition,
    },
    internal_commands::help::Help,
    logger::logger::Logger,
    validations::command_validations::CommandValidations,
};

pub struct SearchCommands {
    pub root: String,
    pub configuration: DevKitConfig,
    pub definition: InternalExecutableDefinition,
}

impl SearchCommands {
    pub fn new(root: String, configuration: DevKitConfig) -> SearchCommands {
        SearchCommands {
            root,
            configuration,
            definition: InternalExecutableDefinition {
                name: "search",
                description: "Retrieve commands that match any search query",
                args: HashMap::from([(
                    "<query>",
                    "A search string to match against command names, descriptions, arguments, or owner",
                )]),
            },
        }
    }

    fn search_internal(&self, query: &str, command: &Box<dyn InternalExecutable>) -> bool {
        let config = command.get_definition();
        if config.name.to_lowercase().contains(query) {
            return true;
        }
        if config.description.to_lowercase().contains(query) {
            return true;
        }
        for (arg, description) in &config.args {
            if arg.to_lowercase().contains(query) || description.to_lowercase().contains(query) {
                return true;
            }
        }
        false
    }

    fn search_external(&self, query: &str, command: &DevKitCommand) -> bool {
        if command.name.to_lowercase().contains(query) {
            return true;
        }
        if command.owner.to_lowercase().contains(query) {
            return true;
        }
        if command
            .location
            .replace(self.root.as_str(), "")
            .to_lowercase()
            .contains(query)
        {
            return true;
        }
        if command.description.to_lowercase().contains(query) {
            return true;
        }
        for (arg, sub_command) in &command.commands {
            if arg.to_lowercase().contains(query) || self.search_command(query, sub_command) {
                return true;
            }
        }
        false
    }

    fn search_command(&self, query: &str, command: &Command) -> bool {
        if command.command.to_lowercase().contains(query)
            || command.description.to_lowercase().contains(query)
        {
            return true;
        }
        false
    }

    fn log_root_results(&self, root_results: &HashMap<String, Command>) {
        let total = root_results.len();
        let plural_appendage = if total == 1 { "" } else { "s" };
        if !root_results.is_empty() {
            Help::log_root_commands(root_results);
        }
        Logger::info(
            format!(
                "Matched {} command{} in your devkit config",
                Logger::blue_bright(total.to_string().as_str()),
                plural_appendage,
            )
            .as_str(),
        );
    }

    fn log_internal_results(
        &self,
        internal_results: &HashMap<String, &Box<dyn InternalExecutable>>,
    ) {
        let total = internal_results.len();
        let plural_appendage = if total == 1 { "" } else { "s" };
        if !internal_results.is_empty() {
            let mut sorted_internals: Vec<&&Box<dyn InternalExecutable>> =
                internal_results.values().collect();
            sort_slice_by_str_key(&mut sorted_internals, |x| &x.get_definition().name);
            Logger::space_around("Internal Commands:");
            for internal in sorted_internals {
                internal.help();
                println!();
            }
        }
        Logger::info(
            format!(
                "Matched {} internal command{}",
                Logger::blue_bright(total.to_string().as_str()),
                plural_appendage,
            )
            .as_str(),
        );
    }

    fn log_external_results(&self, external_commands: &HashMap<String, DevKitCommand>) {
        let total = external_commands.len();
        let plural_appendage = if total == 1 { "" } else { "s" };
        if !external_commands.is_empty() {
            Help::log_external_commands(external_commands);
        }
        Logger::info(
            format!(
                "Matched {} registered command{}",
                Logger::blue_bright(total.to_string().as_str()),
                plural_appendage,
            )
            .as_str(),
        );
    }
}

impl InternalExecutable for SearchCommands {
    fn run(&self, args: Vec<String>, internals: &HashMap<String, Box<dyn InternalExecutable>>) {
        Logger::info("Searching commands");
        if args.is_empty() {
            Logger::exit_with_error("Please specify a search string to query with");
        }
        let query = args.join(" ").to_lowercase();
        let externals = CommandValidations::new(self.root.clone(), self.configuration.clone())
            .collect_and_validate_externals();
        let mut root_results: HashMap<String, Command> = HashMap::new();
        let mut internal_results: HashMap<String, &Box<dyn InternalExecutable>> = HashMap::new();
        let mut external_results: HashMap<String, DevKitCommand> = HashMap::new();
        for (command, script) in &self.configuration.commands {
            if self.search_command(&query, script) {
                root_results.insert(command.clone(), script.clone());
            }
        }
        for (name, command) in internals {
            if self.search_internal(&query, command) {
                internal_results.insert(name.clone(), command);
            }
        }
        for (name, command) in externals {
            if self.search_external(&query, &command) {
                external_results.insert(name, command);
            }
        }
        if root_results.is_empty() && internal_results.is_empty() && external_results.is_empty() {
            Logger::exit_with_info("No matched commands");
        }
        self.log_root_results(&root_results);
        self.log_internal_results(&internal_results);
        self.log_external_results(&external_results);
    }

    fn help(&self) {
        Help::log_internal_command(&self.definition);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
