use crate::{Pass, PassReturn, draw, handle_key_event, init};

pub fn state<'a, S: 'static, T: Default + 'static>(
    pass: Pass<'a>,
    content: &mut dyn for<'b> FnMut(Pass<'b>, &mut T) -> PassReturn<'b, S>,
) -> PassReturn<'a, impl Sized + 'static + use<S, T>> {
    pass.apply(
        content,
        |content| {
            let mut state: T = Default::default();

            let widget_state = init(&mut |pass| content(pass, &mut state));
            (state, widget_state)
        },
        |content, (state, widget_state), area, buffer| {
            draw(&mut |pass| content(pass, state), widget_state, area, buffer);
        },
        |content, (state, widget_state), event| {
            handle_key_event(&mut |pass| content(pass, state), widget_state, event)
        },
    )
}
