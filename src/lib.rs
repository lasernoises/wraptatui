pub mod list_content;
pub mod widget;
pub mod widgets;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{buffer::Buffer, layout::Rect};

pub use widget::*;

pub use Focus::*;

pub fn run<S: 'static>(widget: &mut impl for<'a> FnMut(Pass<'a>) -> PassReturn<S>) -> Result<()> {
    let mut state = init(widget);

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(|frame| {
            let cursor_position = draw(
                widget,
                &mut state,
                Focused,
                frame.area(),
                frame.buffer_mut(),
            );

            if let Some(position) = cursor_position {
                frame.set_cursor_position(position);
            }
        })?;

        let event = event::read()?;

        if let Event::Key(event) = event {
            let handled = handle_key_event(widget, &mut state, event);

            if !handled && event.code == KeyCode::Char('q') {
                break;
            }
        }
    }
    ratatui::restore();
    Ok(())
}

pub struct RatatuiWidgetState(Focusable);

impl WidgetState for RatatuiWidgetState {
    fn reset_focus(&mut self) -> Focusable {
        self.0
    }
}

pub fn ratatui_widget<'a, W: ratatui::widgets::Widget>(
    pass: Pass<'a>,
    focusable: Focusable,
    widget: W,
) -> PassReturn<'a, RatatuiWidgetState> {
    pass.apply(
        widget,
        |_: W| RatatuiWidgetState(focusable),
        |widget: W, _, _, area: Rect, buffer: &mut Buffer| {
            widget.render(area, buffer);
            None
        },
        |_, _, _| false,
    )
}

pub fn ratatui_stateful_widget<'a, W: ratatui::widgets::StatefulWidget>(
    pass: Pass<'a>,
    focusable: Focusable,
    state: &mut W::State,
    widget: W,
) -> PassReturn<'a, RatatuiWidgetState> {
    pass.apply(
        (widget, state),
        |_: (W, &mut W::State)| RatatuiWidgetState(focusable),
        |(widget, state): (W, &mut W::State), _, _, area: Rect, buffer: &mut Buffer| {
            widget.render(area, buffer, state);
            None
        },
        |_, _, _| false,
    )
}
