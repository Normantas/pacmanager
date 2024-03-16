use clap::Parser;
use dialoguer::Input;
use futures_lite::{io::BufReader, prelude::*};

use pacmanager_wrapper::{execute_action, PacManagerAction, PacManagerCommand};

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

fn command_to_pacmanager_command(command: Command, mut package: String) -> PacManagerCommand {
    // Commands which do not require a package
    match command {
        Command::List => return PacManagerCommand::List,
        Command::Update => return PacManagerCommand::Update,
        Command::Upgrade => return PacManagerCommand::Update,
        _ => (),
    };

    // Commands which do require a package
    if package.is_empty() {
        package = Input::new().with_prompt("Package").interact_text().unwrap();
    };

    match command {
        Command::Install => PacManagerCommand::Install(package),
        Command::Reinstall => PacManagerCommand::Reinstall(package),
        Command::Remove => PacManagerCommand::Uninstall(package),
        Command::View => PacManagerCommand::View(package),
        Command::Search => PacManagerCommand::Search(package),
        _ => unreachable!(),
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let pacmanager_command = command_to_pacmanager_command(args.command, args.package);

    let action = PacManagerAction {
        pacmanager_command,
        internal_config: Default::default(),
        non_interactive: true,
        custom_flags: None,
    };

    let mut child = execute_action(action, pacmanager_wrapper::PacManager::Apt)
        .await
        .unwrap();
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

    while let Some(line) = lines.next().await {
        println!("{}", line.unwrap());
    }
}
