use crate::{Focusable, Pass, PassReturn, WidgetState, draw, handle_key_event, init};

pub struct State<S, T> {
    widget_state: S,
    state: T,
}

impl<S: WidgetState, T: 'static> WidgetState for State<S, T> {
    fn reset_focus(&mut self) -> Focusable {
        self.widget_state.reset_focus()
    }
}

pub fn state<'a, S: WidgetState, T: Default + 'static>(
    pass: Pass<'a>,
    content: impl for<'b> FnMut(Pass<'b>, &mut T) -> PassReturn<'b, S>,
) -> PassReturn<'a, State<S, T>> {
    pass.apply(
        content,
        |mut content| {
            let mut state: T = Default::default();

            let widget_state = init(&mut |pass| content(pass, &mut state));
            State {
                widget_state,
                state,
            }
        },
        |mut content, state, focus, area, buffer| {
            draw(
                &mut |pass| content(pass, &mut state.state),
                &mut state.widget_state,
                focus,
                area,
                buffer,
            )
        },
        |mut content, state, event| {
            handle_key_event(
                &mut |pass| content(pass, &mut state.state),
                &mut state.widget_state,
                event,
            )
        },
    )
}
