use crate::{Focusable, Pass, PassReturn, WidgetState, draw, handle_key_event};

pub struct State<S, T> {
    widget_state: S,
    state: T,
}

impl<S: WidgetState, T: 'static> WidgetState for State<S, T> {
    fn reset_focus(&mut self) -> Focusable {
        self.widget_state.reset_focus()
    }
}

pub fn state_with_default<'a, S: WidgetState, T: Default + 'static>(
    pass: Pass<'a>,
    mut content: impl for<'b> FnMut(Pass<'b>, &mut T) -> PassReturn<'b, S>,
) -> PassReturn<'a, State<S, T>> {
    state(pass, &mut (), |_| T::default(), |p, _, s| content(p, s))
}

pub fn state<'a, S: WidgetState, T: 'static, U>(
    pass: Pass<'a>,
    shared: &mut U,
    init: impl FnMut(&mut U) -> T,
    content: impl for<'b> FnMut(Pass<'b>, &mut U, &mut T) -> PassReturn<'b, S>,
) -> PassReturn<'a, State<S, T>> {
    pass.apply(
        (shared, init, content),
        |(shared, mut init, mut content)| {
            let mut state: T = init(shared);

            let widget_state = crate::init(&mut |pass| content(pass, shared, &mut state));
            State {
                widget_state,
                state,
            }
        },
        |(shared, _, mut content), state, focus, area, buffer| {
            draw(
                &mut |pass| content(pass, shared, &mut state.state),
                &mut state.widget_state,
                focus,
                area,
                buffer,
            )
        },
        |(shared, _, mut content), state, event| {
            handle_key_event(
                &mut |pass| content(pass, shared, &mut state.state),
                &mut state.widget_state,
                event,
            )
        },
    )
}
