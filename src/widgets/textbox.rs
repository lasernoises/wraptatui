use crossterm::event::KeyCode;
use ratatui::{
    layout::Position,
    widgets::{Paragraph, Widget},
};
use tui_input::backend::crossterm::EventHandler;

use crate::{Pass, PassReturn};

pub use tui_input::Input;

enum Mode {
    Normal,
    Insert,
}

pub fn textbox<'a>(
    pass: Pass<'a>,
    input: &mut Input,
) -> PassReturn<'a, impl Sized + use<> + 'static> {
    pass.apply(
        input,
        |_| Mode::Normal,
        |input, mode, area, buffer| {
            let width = area.width;
            let scroll = input.visual_scroll(width as usize);

            Paragraph::new(input.value()).render(area, buffer);

            if let Mode::Insert = mode {
                let x = input.visual_cursor().max(scroll) - scroll;

                Some(Position::new(area.x + x as u16, area.y))
            } else {
                None
            }
        },
        |input, mode, event| match mode {
            Mode::Normal => match event.code {
                KeyCode::Char('i') => {
                    *mode = Mode::Insert;
                    true
                }
                _ => false,
            },
            Mode::Insert => {
                if event.code == KeyCode::Esc {
                    *mode = Mode::Normal;
                } else {
                    input.handle_event(&crossterm::event::Event::Key(event));
                }
                true
            }
        },
    )
}
