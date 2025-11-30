pub mod list_content;
pub mod widget;
pub mod widgets;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect};

pub use widget::*;

pub fn run<S: 'static>(widget: &mut impl for<'a> FnMut(Pass<'a>) -> PassReturn<S>) -> Result<()> {
    let mut state = init(widget);

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(|frame| {
            draw(widget, &mut state, frame.area(), frame.buffer_mut());
        })?;
        if matches!(
            event::read()?,
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            })
        ) {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}

pub fn ratatui_widget<'a, W: ratatui::widgets::Widget>(
    pass: Pass<'a>,
    widget: W,
) -> PassReturn<'a, ()> {
    pass.apply(
        widget,
        |_: W| (),
        |widget: W, _: &mut (), area: Rect, buffer: &mut Buffer| {
            widget.render(area, buffer);
        },
    )
}

pub fn ratatui_stateful_widget<'a, W: ratatui::widgets::StatefulWidget>(
    pass: Pass<'a>,
    widget: W,
    state: &mut W::State,
) -> PassReturn<'a, ()> {
    pass.apply(
        (widget, state),
        |_: (W, &mut W::State)| (),
        |(widget, state): (W, &mut W::State), _: &mut (), area: Rect, buffer: &mut Buffer| {
            widget.render(area, buffer, state);
        },
    )
}
