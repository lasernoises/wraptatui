use crossterm::event::KeyEvent;

use crate::{Focusable, Pass, PassReturn, WidgetState, draw, handle_key_event, init};

pub struct State<S>(S);

impl<S: WidgetState> WidgetState for State<S> {
    fn reset_focus(&mut self) -> Focusable {
        Focusable::Yes
    }
}

pub fn with_key_handler<'a, S: WidgetState, T>(
    pass: Pass<'a>,
    shared: &mut T,
    handler: impl FnMut(&mut T, KeyEvent) -> bool,
    content: impl for<'b> FnMut(Pass<'b>, &mut T) -> PassReturn<'b, S>,
) -> PassReturn<'a, State<S>> {
    pass.apply(
        (shared, handler, content),
        |(shared, _, mut content)| State(init(&mut |p| content(p, shared))),
        |(shared, _, mut content), State(state), focus, area, buffer| {
            draw(&mut |p| content(p, shared), state, focus, area, buffer)
        },
        |(shared, mut handler, mut content), State(state), event| {
            handle_key_event(&mut |p| content(p, shared), state, event) || handler(shared, event)
        },
    )
}
