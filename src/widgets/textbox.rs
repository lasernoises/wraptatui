use ratatui::{
    layout::Position,
    widgets::{Paragraph, Widget},
};
use tui_input::backend::crossterm::EventHandler;

use crate::{Pass, PassReturn};

pub use tui_input::Input;

pub fn textbox<'a>(
    pass: Pass<'a>,
    input: &mut Input,
) -> PassReturn<'a, impl Sized + use<> + 'static> {
    pass.apply(
        input,
        |_| (),
        |input, _, area, buffer| {
            let width = area.width;
            let scroll = input.visual_scroll(width as usize);

            Paragraph::new(input.value()).render(area, buffer);

            let x = input.visual_cursor().max(scroll) - scroll;

            Some(Position::new(area.x + x as u16, area.y))
        },
        |input, _, event| {
            input
                .handle_event(&crossterm::event::Event::Key(event))
                .is_some()
        },
    )
}
