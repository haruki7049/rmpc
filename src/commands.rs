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
