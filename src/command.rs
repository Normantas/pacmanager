use dialoguer::Input;
use pacmanager_wrapper::PacManagerCommand;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Command {
    // Package management
    Install,
    Reinstall,
    Remove,
    // System maintenance
    Update,
    Upgrade,
    // Package search
    List,
    Search,
    View,
}

// TODO Make this not spaghetti code, or just remove it entirely
pub fn command_to_pacmanager_command(command: Command, mut package: String) -> (PacManagerCommand, bool) {
    // Commands which do not require a package
    match command {
        Command::List => return (PacManagerCommand::List, false),
        Command::Update => return (PacManagerCommand::Update, false),
        Command::Upgrade => return (PacManagerCommand::Update, false),
        _ => (),
    };

    // Commands which do require a package
    if package.is_empty() {
        package = Input::new().with_prompt("Package").interact_text().unwrap();
    };

    match command {
        Command::Install => (PacManagerCommand::Install(package), true),
        Command::Reinstall => (PacManagerCommand::Reinstall(package), true),
        Command::Remove => (PacManagerCommand::Uninstall(package), true),
        Command::View => (PacManagerCommand::View(package), false),
        Command::Search => (PacManagerCommand::Search(package), false),
        _ => unreachable!(),
    }
}