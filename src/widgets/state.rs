use ratatui::{buffer::Buffer, layout::Rect};

use crate::{Pass, PassReturn, Widget, draw, init};

pub fn state<'a, S: 'static, T: Default + 'static>(
    pass: Pass<'a>,
    content: &mut dyn for<'b> FnMut(Pass<'b>, &mut T) -> PassReturn<'b, S>,
) -> PassReturn<'a, impl Sized + 'static + use<S, T>> {
    pass.apply(
        content,
        Widget {
            init: |content: &mut dyn for<'b> FnMut(Pass<'b>, &mut T) -> PassReturn<'b, S>| {
                let mut state: T = Default::default();

                let widget_state = init(&mut |pass| content(pass, &mut state));
                (state, widget_state)
            },
            draw: |content: &mut dyn for<'b> FnMut(Pass<'b>, &mut T) -> PassReturn<'b, S>,
                   (state, widget_state): &mut (T, S),
                   area: Rect,
                   buffer: &mut Buffer| {
                draw(&mut |pass| content(pass, state), widget_state, area, buffer);
            },
        },
    )
}
