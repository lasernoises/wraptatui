use ratatui::{
    layout::Position,
    widgets::{Paragraph, Widget},
};
use tui_input::backend::crossterm::EventHandler;

use crate::{Focusable, Pass, PassReturn, WidgetState};

pub use tui_input::Input;

pub struct State;

impl WidgetState for State {
    fn reset_focus(&mut self) -> Focusable {
        Focusable::Yes
    }
}

pub fn textbox<'a>(pass: Pass<'a>, input: &mut Input) -> PassReturn<'a, State> {
    pass.apply(
        input,
        |_| State,
        |input, _, _, area, buffer| {
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
