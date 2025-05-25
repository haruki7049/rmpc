use mpd::{Client, Query};
use clap::Parser;
use std::net::{IpAddr, Ipv4Addr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();

    let mut c = Client::connect((args.ip, args.port)).unwrap();
    println!("version: {:?}", c.version);
    println!("status: {:?}", c.status());
    println!("stuff: {:?}", c.find(&Query::new(), (1, 2)));

    let now_playing = c.currentsong()?;
    if let Some(song) = now_playing {
        println!("Metadata:");
        for (k, v) in (c.readcomments(song)?).flatten() {
            println!("{}: {}", k, v);
        }
    } else {
        println!("No song playing.");
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
}
