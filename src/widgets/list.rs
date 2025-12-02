use ratatui::layout::{Direction, Layout};

use crate::{
    Pass, PassReturn, draw, handle_key_event,
    list_content::{ConstraintsIter, ListContent},
};

pub fn list<'a, S: 'static>(
    pass: Pass<'a>,
    direction: Direction,
    content: &mut dyn ListContent<State = S>,
) -> PassReturn<'a, impl Sized + 'static + use<S>> {
    pass.apply(
        content,
        |content| content.init(),
        |content, state, area, buffer| {
            let layout = Layout::new(direction, ConstraintsIter(content));
            let areas = layout.split(area);
            let mut areas = areas.iter();

            let mut position = None;

            content.all(state, &mut |widget, focused| {
                let area = *areas.next().unwrap();

                if let Some(widget_position) = draw(widget, &mut (), area, buffer)
                    && focused
                {
                    position = Some(widget_position);
                }
            });

            position
        },
        |content, state, event| {
            let mut handled = false;

            content.all(state, &mut |widget, focused| {
                if focused {
                    handled = handle_key_event(widget, &mut (), event);
                }
            });

            handled
        },
    )
}

pub fn hlist<'a, S: 'static>(
    pass: Pass<'a>,
    content: &mut dyn ListContent<State = S>,
) -> PassReturn<'a, impl Sized + 'static + use<S>> {
    list(pass, Direction::Horizontal, content)
}

pub fn vlist<'a, S: 'static>(
    pass: Pass<'a>,
    content: &mut dyn ListContent<State = S>,
) -> PassReturn<'a, impl Sized + 'static + use<S>> {
    list(pass, Direction::Vertical, content)
}
