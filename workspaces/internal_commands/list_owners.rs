use std::collections::{HashMap, HashSet};

use alphanumeric_sort::sort_str_slice;

use crate::{
    executables::{
        intenal_executable::InternalExecutable,
        internal_executable_definition::InternalExecutableDefinition,
    },
    internal_commands::help::Help,
    logger::logger::Logger,
    repokit::interfaces::{RepoKitCommand, RepoKitConfig},
    validations::command_validations::CommandValidations,
};

pub struct ListOwners {
    pub root: String,
    pub configuration: RepoKitConfig,
    pub definition: InternalExecutableDefinition,
}

impl ListOwners {
    pub fn new(root: String, configuration: RepoKitConfig) -> ListOwners {
        ListOwners {
            root,
            configuration,
            definition: InternalExecutableDefinition {
                name: "owners",
                description: "Lists all registered command owners",
                args: HashMap::from([]),
            },
        }
    }

    fn collect_registered_commands(&self) -> HashMap<String, RepoKitCommand> {
        let validators = CommandValidations::new(self.root.clone(), self.configuration.clone());
        validators.collect_and_validate_externals()
    }
}

impl InternalExecutable for ListOwners {
    fn run(&self, _: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        let registered_commands = self.collect_registered_commands();
        Logger::info("Listing all command owners");
        let mut owners: HashSet<String> = HashSet::new();
        for (_, command) in registered_commands {
            if !command.owner.is_empty() {
                owners.insert(command.owner);
            }
        }
        if owners.is_empty() {
            return Logger::exit_with_info("No owners found");
        }
        println!();
        let mut list: Vec<String> = owners.into_iter().collect();
        sort_str_slice(&mut list);
        for (index, owner) in list.iter().enumerate() {
            println!(
                "{}{}",
                Logger::indent(None),
                Logger::cyan(format!("{}. {}", index + 1, &owner).as_str()),
            );
        }
        println!();
    }

    fn help(&self) {
        Help::log_internal_command(&self.definition);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
