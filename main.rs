use termion::{clear, cursor, terminal_size};
use termion::raw::IntoRawMode;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::{Arc, Mutex};
use std::io::{self, Write, stdout};
use std::time::{Duration, Instant};
use std::thread;



fn boarder<W>(stdout: &mut W) -> io::Result<()>
where
    W: Write,
{
    let (width, height) = terminal_size()?;
    let horizontal = "=".repeat(width as usize - 1);
    let vertical = "|";
    write!(
        stdout,
        "{}{}{}\n",
        cursor::Goto(1, 1),
        horizontal,
        cursor::Goto(1, height - 1)
    )?;

    for i in 2..height - 1 {
        write!(
            stdout,
            "{}{}{}{}\n",
            cursor::Goto(1, i),
            vertical,
            cursor::Goto(width - 1, i),
            vertical
        )?;
    }

    write!(
        stdout,
        "{}{}{}\n",
        cursor::Goto(1, height - 1),
        horizontal,
        cursor::Goto(width - 1, height - 2)
    )?;

    Ok(())
}

fn main() -> io::Result<()> {
    loop{
        let mut stdout = io::stdout().into_raw_mode()?;
        write!(stdout, "{}", clear::All)?;
        boarder(&mut stdout)?;
        stdout.flush()?;
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())

}
