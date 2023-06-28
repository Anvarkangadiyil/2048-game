mod action;
mod game;
mod display;
use std::{time::Duration, process::exit};
use crossterm::{event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers}};
use game::*;



fn main() -> Result<(), std::io::Error> {
    
    
    let mut game = Game::new();

    crossterm::terminal::enable_raw_mode()?;

    game.display();
    
    loop {
        if crossterm::event::poll(Duration::from_millis(50))?{
        //key listener    
        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => break,
            Event::Key(KeyEvent { code, .. }) => {
                if action::has_won(&mut game) {
                    display::to_display_won();
                } else if action::has_defeat(&mut game) {
                    display::to_display_lose();
                    exit(0);
                } else {
                    action::movement(&mut game, code);
                }
            }
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Mouse(_) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => todo!(),
        }
    }
}

    crossterm::terminal::disable_raw_mode()
}
