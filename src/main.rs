use clap::Parser;
use clap::Subcommand;
use clap_complete::Shell;
use mpd::Client;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();

    match args.shell_completion {
        Some(shell) => {
            self::completion::display(shell);
            return Ok(());
        }
        None => (),
    }

    let mut c: Client = Client::connect((args.ip, args.port)).unwrap();
    let command: Command = match args.command {
        Some(command) => command,
        None => {
            rmpc::commands::status(&mut c)?;
            return Ok(());
        }
    };

    match command {
        Command::Status => rmpc::commands::status(&mut c)?,
        Command::Toggle => rmpc::commands::toggle(&mut c)?,
        Command::Play => rmpc::commands::play(&mut c)?,
        Command::Listall => rmpc::commands::listall(&mut c)?,
        Command::Add { filepath: s } => rmpc::commands::add(&mut c, s)?,
        Command::Stats => rmpc::commands::stats(&mut c)?,
        Command::Stop => rmpc::commands::stop(&mut c)?,
        Command::Queue(v) => match v {
            QueueCommand::List => rmpc::commands::queue::list(&mut c)?,
            QueueCommand::NextTrack => rmpc::commands::queue::next_track(&mut c)?,
            QueueCommand::Clear => rmpc::commands::queue::clear(&mut c)?,
        },
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

    #[arg(long)]
    shell_completion: Option<Shell>,
}

#[derive(Default, Debug, Clone, Subcommand)]
enum Command {
    #[default]
    Status,
    Toggle,
    Play,
    Listall,
    Add {
        filepath: PathBuf,
    },
    Stats,
    Stop,

    #[clap(subcommand)]
    Queue(QueueCommand),
}

#[derive(Debug, Clone, Subcommand)]
enum QueueCommand {
    List,
    NextTrack,
    Clear,
}

mod completion {
    use clap_complete::{Generator, generate};
    use clap::CommandFactory;
    use super::CLIArgs;

    pub fn display<G: Generator>(generator: G) {
        generate(
            generator,
            &mut CLIArgs::command(),
            env!("CARGO_PKG_NAME"),
            &mut std::io::stdout(),
        );
    }
}
