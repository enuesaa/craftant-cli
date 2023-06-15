use crate::service::registry::{CommandDef, Registry};
use inquire::Text;

pub fn add_command_handler() {
    let name = Text::new("Please enter command name to register").prompt().unwrap();
    let command = Text::new("Which shell command would you like to run?").prompt().unwrap();
    let description = Text::new("Please enter description").prompt().unwrap();

    let mut registry = Registry::new();
    let command = CommandDef {
        name: name.clone(),
        command: command.clone(),
        description: description.clone(),
    };
    registry.create_command(command);
}
