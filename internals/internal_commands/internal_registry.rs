use std::collections::HashMap;

use crate::{
    executables::intenal_executable::InternalExecutable,
    internal_commands::{
        list_commands::ListCommands, list_owners::ListOwners, locate_command::LocateCommand,
        onboarder::Onboarder, register_command::RegisterCommand, search_commands::SearchCommands,
        upgrade_repokit::UpgradeRepoKit,
    },
    repokit::interfaces::RepoKitConfig,
};

pub struct InternalRegistry {
    root: String,
    configuration: RepoKitConfig,
}

impl InternalRegistry {
    pub fn new(root: String, configuration: RepoKitConfig) -> InternalRegistry {
        InternalRegistry {
            root,
            configuration,
        }
    }

    pub fn get_all(&self) -> HashMap<String, Box<dyn InternalExecutable>> {
        let internals: [Box<dyn InternalExecutable>; 7] = [
            Box::new(Onboarder::new(
                self.root.clone(),
                self.configuration.clone(),
            )),
            Box::new(ListCommands::new(
                self.root.clone(),
                self.configuration.clone(),
            )),
            Box::new(SearchCommands::new(
                self.root.clone(),
                self.configuration.clone(),
            )),
            Box::new(ListOwners::new(
                self.root.clone(),
                self.configuration.clone(),
            )),
            Box::new(LocateCommand::new(
                self.root.clone(),
                self.configuration.clone(),
            )),
            Box::new(RegisterCommand::new(
                self.root.clone(),
                self.configuration.clone(),
            )),
            Box::new(UpgradeRepoKit::new(
                self.root.clone(),
                self.configuration.clone(),
            )),
        ];
        HashMap::from(internals.map(|x| (x.get_definition().name.to_string(), x)))
    }
}
