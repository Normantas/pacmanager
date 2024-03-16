use clap::Parser;
use dialoguer::Input;
use futures_lite::{io::BufReader, prelude::*};

use pacmanager_wrapper::{execute_action, PacManager, PacManagerAction, PacManagerCommand};

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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_enum, index = 1)]
    command: Command,

    #[arg(index = 2, default_value_t = String::from(""))]
    package: String,
}

fn command_to_pacmanager_command(command: Command, mut package: String) -> (PacManagerCommand, bool) {
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

fn get_package_manager() -> PacManager {
    let identify = whatadistro::identify().unwrap();
    if identify.is_similar("arch") {
        PacManager::Pacman
    } else if identify.is_similar("debian") {
        PacManager::Apt
    } else if identify.is_similar("rhel") || identify.is_similar("fedora") {
        PacManager::Yum
    } else {
        panic!("Unsupported package manager")
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let (pacmanager_command, should_be_noninteractive) = command_to_pacmanager_command(args.command, args.package);

    let action = PacManagerAction {
        pacmanager_command,
        internal_config: Default::default(),
        non_interactive: should_be_noninteractive,
        custom_flags: None,
    };

    let mut child = execute_action(action, get_package_manager())
        .await
        .unwrap();
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

    while let Some(line) = lines.next().await {
        println!("{}", line.unwrap());
    }
}
