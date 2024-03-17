use clap::Parser;
use futures_lite::{io::BufReader, prelude::*};

mod command;
use command::{command_to_pacmanager_command, Command};

use pacmanager_wrapper::{execute_action, PacManager, PacManagerAction};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_enum, index = 1)]
    command: Command,

    #[arg(index = 2, default_value_t = String::from(""))]
    package: String,

    /// Override to APT package manager
    #[arg(long, default_value_t = false, conflicts_with_all = ["apk", "yum", "pacman"])]
    apt: bool,

    /// Override to APK package manager
    #[arg(long, default_value_t = false, conflicts_with_all = ["apt", "yum", "pacman"])]
    apk: bool,

    /// Override to YUM package manager
    #[arg(long, default_value_t = false, conflicts_with_all = ["apt", "apk", "pacman"])]
    yum: bool,

    /// Override to PACMAN package manager
    #[arg(long, default_value_t = false, conflicts_with_all = ["apt", "apk", "yum"])]
    pacman: bool,
}

// TODO Make this not spaghetti code
fn get_package_manager(args: &Args) -> PacManager {
    if args.apt {
        return PacManager::Apt
    } else if args.apk {
        return PacManager::Apk
    } else if args.yum {
        return PacManager::Yum
    } else if args.pacman {
        return PacManager::Pacman
    }

    let identify = whatadistro::identify().unwrap();
    if identify.is_similar("arch") {
        PacManager::Pacman
    } else if identify.is_similar("debian") {
        PacManager::Apt
    } else if identify.is_similar("rhel") || identify.is_similar("fedora") {
        PacManager::Yum
    } else {
        panic!("It appers you have an unsupported package manager! If you do have a supported package manager, try using an override flag (like \"--pacman\")")
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let pacmanager = get_package_manager(&args);
    let (pacmanager_command, should_be_noninteractive) = command_to_pacmanager_command(args.command, args.package);

    let action = PacManagerAction {
        pacmanager_command,
        internal_config: Default::default(),
        non_interactive: should_be_noninteractive,
        custom_flags: None,
    };

    let mut child = execute_action(action, pacmanager)
        .await
        .unwrap();
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

    while let Some(line) = lines.next().await {
        println!("{}", line.unwrap());
    }
}
