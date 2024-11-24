use crossterm::terminal;
use std::error::Error;
use std::os::fd::AsRawFd;
use termios::{tcsetattr, Termios, ECHO, ICANON, OPOST, TCSANOW};

pub fn init_terminal() -> Result<Termios, Box<dyn Error>> {
    let stdin_fd = std::io::stdin().as_raw_fd();
    let original_termios = Termios::from_fd(stdin_fd)?;
    let mut raw_termios = original_termios;

    // enable raw mode
    terminal::enable_raw_mode()?;

    // enable OPOST and disable ICANON and ECHO
    raw_termios.c_lflag &= !(ICANON | ECHO);
    raw_termios.c_oflag |= OPOST;
    tcsetattr(stdin_fd, TCSANOW, &raw_termios)?;

    Ok(original_termios)
}

pub fn reset_terminal(original_termios: &Termios) -> Result<(), Box<dyn Error>> {
    let stdin_fd = std::io::stdin().as_raw_fd();
    tcsetattr(stdin_fd, TCSANOW, original_termios)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
