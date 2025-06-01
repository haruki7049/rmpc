use crossterm::ExecutableCommand;
use crossterm::style::Attribute;
use crossterm::style::Color;
use crossterm::style::Print;
use crossterm::style::ResetColor;
use crossterm::style::SetAttribute;
use crossterm::style::SetForegroundColor;
use mpd::Client;
use mpd::Song;
use mpd::State;
use std::path::PathBuf;

pub fn status(c: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn toggle(c: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    c.toggle_pause()?;
    self::status(c)?;
    Ok(())
}

pub fn play(c: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    c.play()?;
    self::status(c)?;
    Ok(())
}

pub fn listall(c: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    let songs: Vec<Song> = c.listall()?;
    for song in songs {
        println!("{}", song.file);
    }

    Ok(())
}

pub fn add(c: &mut Client, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Allow fullpath for the path argument to compare filename
    // Example:
    // [src/commands.rs:196:9] &name = "/home/haruki/Music/Ash Crow [FLAC]/01 - 灰よ.flac"
    // [src/commands.rs:196:9] &song = Song {
    //     file: "音楽産業廃棄物〜P-MODEL OR DIE [FLAC]/10 - DUSToidよ歩行は快適か？.flac",
    //     name: None,
    //     title: None,
    //     last_mod: None,
    //     artist: None,
    //     duration: None,
    //     place: None,
    //     range: None,
    //     tags: [],
    // }

    let songs: Vec<Song> = c.listall()?;
    let name: &str = path.to_str().unwrap_or_default();

    for song in songs {
        dbg!(&name, &song);
        if song.file == name {
            c.push(song)?;
        }
    }

    Ok(())
}

pub fn stats(c: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    let stats = c.stats()?;
    let artists = stats.artists;
    let albums = stats.albums;
    let songs = stats.songs;
    let playtime = time::Duration::try_from(stats.playtime)?;
    let uptime = time::Duration::try_from(stats.uptime)?;
    let db_update = time::Duration::try_from(stats.db_update)?;
    let db_playtime = time::Duration::try_from(stats.db_playtime)?;

    let mut stdout: std::io::Stdout = std::io::stdout();

    stdout
        .execute(SetForegroundColor(Color::White))?
        .execute(SetAttribute(Attribute::Bold))?
        .execute(Print("Artists: "))?
        .execute(ResetColor)?;

    stdout.execute(Print(artists))?.execute(Print("\n"))?;

    stdout
        .execute(SetForegroundColor(Color::White))?
        .execute(SetAttribute(Attribute::Bold))?
        .execute(Print("Albums: "))?
        .execute(ResetColor)?;

    stdout.execute(Print(albums))?.execute(Print("\n"))?;

    stdout
        .execute(SetForegroundColor(Color::White))?
        .execute(SetAttribute(Attribute::Bold))?
        .execute(Print("Songs: "))?
        .execute(ResetColor)?;

    stdout.execute(Print(songs))?.execute(Print("\n"))?;

    stdout.execute(Print("\n"))?;

    stdout
        .execute(SetForegroundColor(Color::White))?
        .execute(SetAttribute(Attribute::Bold))?
        .execute(Print("Play Time: "))?
        .execute(ResetColor)?;

    stdout.execute(Print(playtime))?.execute(Print("\n"))?;

    stdout
        .execute(SetForegroundColor(Color::White))?
        .execute(SetAttribute(Attribute::Bold))?
        .execute(Print("Uptime: "))?
        .execute(ResetColor)?;

    stdout.execute(Print(uptime))?.execute(Print("\n"))?;

    stdout
        .execute(SetForegroundColor(Color::White))?
        .execute(SetAttribute(Attribute::Bold))?
        .execute(Print("DB Updated: "))?
        .execute(ResetColor)?;

    stdout.execute(Print(db_update))?.execute(Print("\n"))?;

    stdout
        .execute(SetForegroundColor(Color::White))?
        .execute(SetAttribute(Attribute::Bold))?
        .execute(Print("DB Play Time: "))?
        .execute(ResetColor)?;

    stdout.execute(Print(db_playtime))?.execute(Print("\n"))?;

    Ok(())
}

pub mod queue {
    use mpd::Client;
    use mpd::Song;
    use mpd::Status;

    pub fn list(c: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
        let status: Status = c.status()?;
        let now_position: u32 = status.song.ok_or("No playing song found")?.pos + 1; // The pos variable starts with zero

        let queue_list: Vec<Song> = c.queue()?;
        let mut counter: u32 = 1;
        for song in queue_list {
            if counter == now_position {
                println!("Now playing: {}", song.file);
            } else {
                println!("             {}", song.file);
            }
            counter += 1;
        }

        Ok(())
    }

    pub fn next_track(c: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
        let status: Status = c.status()?;
        let now_position: usize = status.song.ok_or("No playing song found")?.pos.try_into()?; // The pos variable starts with zero, but queue_list also starts with zero

        let queue_list: Vec<Song> = c.queue()?;
        let next_song: Option<&Song> = queue_list.get(now_position + 1);
        match next_song {
            Some(song) => println!("{}", song.file),
            None => println!("No songs found"),
        }

        Ok(())
    }
}
