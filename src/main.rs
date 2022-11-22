use std::io::stdout;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseButton,
        MouseEventKind,
    },
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

fn main() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |p| {
        disable_raw_mode().unwrap();
        hook(p);
    }));
    enable_raw_mode().unwrap();
    execute!(stdout(), EnableMouseCapture, Hide).unwrap();
    let mut colour = Color::White;
    loop {
        match event::read().unwrap() {
            Event::Mouse(evt) => match evt.kind {
                MouseEventKind::Down(button) | MouseEventKind::Drag(button) => {
                    let mut colour = colour;
                    match button {
                        MouseButton::Right => {
                            colour = Color::Reset;
                        }
                        _ => {}
                    }
                    execute!(
                        stdout(),
                        MoveTo(evt.column, evt.row),
                        SetBackgroundColor(colour),
                        Print(" "),
                    )
                    .unwrap();
                }
                _ => {}
            },
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    execute!(stdout(), ResetColor, Clear(ClearType::All)).unwrap();
                }
                KeyCode::Char('1') => {
                    colour = Color::White;
                }
                KeyCode::Char('2') if key.modifiers.contains(KeyModifiers::ALT) => {
                    colour = Color::DarkRed;
                }
                KeyCode::Char('2') => {
                    colour = Color::Red;
                }
                KeyCode::Char('3') if key.modifiers.contains(KeyModifiers::ALT) => {
                    colour = Color::DarkBlue;
                }
                KeyCode::Char('3') => {
                    colour = Color::Blue;
                }
                KeyCode::Char('4') if key.modifiers.contains(KeyModifiers::ALT) => {
                    colour = Color::DarkGreen;
                }
                KeyCode::Char('4') => {
                    colour = Color::Green;
                }
                KeyCode::Char('5') if key.modifiers.contains(KeyModifiers::ALT) => {
                    colour = Color::DarkYellow;
                }
                KeyCode::Char('5') => {
                    colour = Color::Yellow;
                }
                KeyCode::Char('6') if key.modifiers.contains(KeyModifiers::ALT) => {
                    colour = Color::DarkMagenta;
                }
                KeyCode::Char('6') => {
                    colour = Color::Magenta;
                }
                KeyCode::Char('7') if key.modifiers.contains(KeyModifiers::ALT) => {
                    colour = Color::DarkGrey;
                }
                KeyCode::Char('7') => {
                    colour = Color::Grey;
                }
                KeyCode::Char('8') => {
                    colour = Color::Black;
                }
                _ => {}
            },

            _ => {}
        }
    }
    execute!(
        stdout(),
        DisableMouseCapture,
        ResetColor,
        Clear(ClearType::All),
        Show
    )
    .unwrap();
    disable_raw_mode().unwrap();
}
