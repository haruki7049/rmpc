use clap::Parser;
use clap::Subcommand;
use crossterm::ExecutableCommand;
use crossterm::style::Attribute;
use crossterm::style::Color;
use crossterm::style::Print;
use crossterm::style::ResetColor;
use crossterm::style::SetAttribute;
use crossterm::style::SetForegroundColor;
use mpd::Client;
use mpd::State;
use std::net::{IpAddr, Ipv4Addr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();
    let mut c: Client = Client::connect((args.ip, args.port)).unwrap();

    let command: Command = match args.command {
        Some(command) => command,
        None => {
            status(&mut c)?;
            return Ok(());
        }
    };

    match command {
        Command::Status => status(&mut c)?,
    }

    Ok(())
}

fn status(c: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    // Gets Stdout
    let mut stdout: std::io::Stdout = std::io::stdout();

    let status = c.status()?;
    let volume = status.volume;
    let repeat = status.repeat;
    let random = status.random;
    let single = status.single;
    let consume = status.consume;
    let pos: u32 = status.song.unwrap_or_default().pos + 1; // pos starts with 0
    let queue_len: u32 = status.queue_len;
    let playing_state = status.state;
    let currentsong = c.currentsong()?.unwrap_or_default();

    // Displays only Volume, Repeat, Random, Single & Consume status
    if playing_state == State::Stop {
        stdout
            .execute(SetForegroundColor(Color::White))?
            .execute(SetAttribute(Attribute::Bold))?
            .execute(Print("Volume: "))?
            .execute(ResetColor)?;

        stdout.execute(Print(volume))?.execute(Print("    "))?;

        stdout
            .execute(SetForegroundColor(Color::White))?
            .execute(SetAttribute(Attribute::Bold))?
            .execute(Print("Repeat: "))?
            .execute(ResetColor)?;

        stdout.execute(Print(repeat))?.execute(Print("    "))?;

        stdout
            .execute(SetForegroundColor(Color::White))?
            .execute(SetAttribute(Attribute::Bold))?
            .execute(Print("Random: "))?
            .execute(ResetColor)?;

        stdout.execute(Print(random))?.execute(Print("    "))?;

        stdout
            .execute(SetForegroundColor(Color::White))?
            .execute(SetAttribute(Attribute::Bold))?
            .execute(Print("Single: "))?
            .execute(ResetColor)?;

        stdout.execute(Print(single))?.execute(Print("    "))?;

        stdout
            .execute(SetForegroundColor(Color::White))?
            .execute(SetAttribute(Attribute::Bold))?
            .execute(Print("Consume: "))?
            .execute(ResetColor)?;

        stdout.execute(Print(consume))?.execute(Print("    "))?;

        stdout.execute(Print("\n"))?;

        // Early return
        return Ok(());
    }

    // Displays Artists & Title info
    {
        stdout
            .execute(Print(currentsong.artist.unwrap_or_default()))?
            .execute(ResetColor)?;

        stdout.execute(Print(" - "))?;

        stdout
            .execute(Print(currentsong.title.unwrap_or_default()))?
            .execute(ResetColor)?;

        stdout.execute(Print("\n"))?;
    }

    // Displays playing status, number of track, Elapsed time (with percentage)
    {
        // [playing] or [paused]
        stdout.execute(Print("["))?;
        match playing_state {
            State::Play => stdout.execute(Print("playing"))?.execute(ResetColor)?,
            State::Pause => stdout.execute(Print("paused"))?.execute(ResetColor)?,
            State::Stop => unreachable!(),
        };
        stdout.execute(Print("]"))?.execute(Print("    "))?;

        // Now position of queue
        stdout
            .execute(Print("#"))?
            .execute(Print(pos))?
            .execute(Print("/"))?
            .execute(Print(queue_len))?;

        stdout.execute(Print("    "))?;

        // time current song played, and total song duration
        let (current_duration, total_duration) = status.time.unwrap_or_default();
        stdout.execute(Print(format!(
            "{:?}/{:?}",
            current_duration, total_duration
        )))?;

        stdout.execute(Print("\n"))?;
    }

    // Displays Volume, Repeat, Random, Single & Consume status
    {
        stdout
            .execute(SetForegroundColor(Color::White))?
            .execute(SetAttribute(Attribute::Bold))?
            .execute(Print("Volume: "))?
            .execute(ResetColor)?;

        stdout.execute(Print(volume))?.execute(Print("    "))?;

        stdout
            .execute(SetForegroundColor(Color::White))?
            .execute(SetAttribute(Attribute::Bold))?
            .execute(Print("Repeat: "))?
            .execute(ResetColor)?;

        stdout.execute(Print(repeat))?.execute(Print("    "))?;

        stdout
            .execute(SetForegroundColor(Color::White))?
            .execute(SetAttribute(Attribute::Bold))?
            .execute(Print("Random: "))?
            .execute(ResetColor)?;

        stdout.execute(Print(random))?.execute(Print("    "))?;

        stdout
            .execute(SetForegroundColor(Color::White))?
            .execute(SetAttribute(Attribute::Bold))?
            .execute(Print("Single: "))?
            .execute(ResetColor)?;

        stdout.execute(Print(single))?.execute(Print("    "))?;

        stdout
            .execute(SetForegroundColor(Color::White))?
            .execute(SetAttribute(Attribute::Bold))?
            .execute(Print("Consume: "))?
            .execute(ResetColor)?;

        stdout.execute(Print(consume))?.execute(Print("    "))?;

        stdout.execute(Print("\n"))?;
    }
    Ok(())
}

#[derive(Debug, Parser)]
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
