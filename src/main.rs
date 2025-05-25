use rmpc::commands;

use clap::Parser;
use clap::Subcommand;
use mpd::Client;
use std::net::{IpAddr, Ipv4Addr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();
    let mut c: Client = Client::connect((args.ip, args.port)).unwrap();

    let command: Command = match args.command {
        Some(command) => command,
        None => {
            commands::status(&mut c)?;
            return Ok(());
        }
    };

    match command {
        Command::Status => commands::status(&mut c)?,
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about)]
struct CLIArgs {
    /// An IP which references your MPD server
    #[arg(short, long, default_value_t = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
    ip: IpAddr,

    /// An IP which references your MPD server
    #[arg(short, long, default_value_t = 6600)]
    port: u16,

    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Default, Debug, Clone, Subcommand)]
enum Command {
    #[default]
    Status,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Status => write!(f, "status"),
        }
    }
}
