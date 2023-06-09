use crate::{
    commands::{
        edit_config::EditConfigCommand, help::HelpCommand, install::InstallCommand,
        list_mods::ListCommand, print_config::PrintConfigCommand, quit::QuitCommand,
        search::SearchCommand, uninstall::UninstallCommand,
    },
    lib::io::io_helper::print_middle,
};

use super::command::Command;

use indexmap::IndexMap;

pub(crate) fn create_command_handler() -> IndexMap<&'static str, Box<dyn Command>> {
    let mut dispatcher: IndexMap<&'static str, Box<dyn Command>> = IndexMap::new();
    dispatcher.insert("sS", Box::new(SearchCommand));
    dispatcher.insert("S", Box::new(InstallCommand));
    dispatcher.insert("R", Box::new(UninstallCommand));
    dispatcher.insert("pconfig", Box::new(PrintConfigCommand));
    dispatcher.insert("config", Box::new(EditConfigCommand));
    dispatcher.insert("l", Box::new(ListCommand));
    dispatcher.insert("h", Box::new(HelpCommand));
    dispatcher.insert("q", Box::new(QuitCommand));

    // Add more commands here

    dispatcher
}

pub(crate) fn print_help_menu() {
    let dispatcher = create_command_handler();

    print_middle(
        "==============================================",
        &format!("Modify {}", env!("CARGO_PKG_VERSION").trim_matches('"')),
    );

    for (command_letter, command) in dispatcher.iter() {
        println!("• {} - {}", command_letter, command.description());
    }
}
