use ratatui::{
    buffer::Buffer,
    layout::{Direction, Layout, Rect},
};

use crate::{
    Pass, PassReturn, draw,
    list_content::{ConstraintsIter, ListContent},
};

pub fn list<'a, S: 'static>(
    pass: Pass<'a>,
    direction: Direction,
    content: &mut dyn ListContent<State = S>,
) -> PassReturn<'a, impl Sized + 'static + use<S>> {
    pass.apply(
        content,
        |content: &mut dyn ListContent<State = S>| content.init(),
        |content: &mut dyn ListContent<State = S>,
         state: &mut S,
         area: Rect,
         buffer: &mut Buffer| {
            let layout = Layout::new(direction, ConstraintsIter(content));
            let areas = layout.split(area);
            let mut areas = areas.iter();

            content.all(state, &mut |widget| {
                let area = *areas.next().unwrap();

                draw(widget, &mut (), area, buffer);
            });
        },
        |_, _, _| false,
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
