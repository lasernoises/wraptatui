use ratatui::layout::{Direction, Layout};

use crate::{
    Focus, Focusable, Pass, PassReturn, WidgetState, draw, handle_key_event,
    list_content::{ConstraintsIter, DummyWidgetState, ListContent, ListContentState},
};

pub struct State<S> {
    list_content_state: S,
}

impl<S: ListContentState> WidgetState for State<S> {
    fn reset_focus(&mut self) -> Focusable {
        self.list_content_state.reset_focus()
    }
}

pub fn list<'a, S: ListContentState>(
    pass: Pass<'a>,
    direction: Direction,
    content: &mut dyn ListContent<State = S>,
) -> PassReturn<'a, State<S>> {
    pass.apply(
        content,
        |content| {
            let mut list_content_state = content.init();
            list_content_state.reset_focus();
            State { list_content_state }
        },
        |content, state, focus, area, buffer| {
            let layout = Layout::new(direction, ConstraintsIter(content));
            let areas = layout.split(area);
            let mut areas = areas.iter();

            let mut position = None;

            content.all(&mut state.list_content_state, &mut |widget, _| {
                let area = *areas.next().unwrap();

                if let Some(widget_position) =
                    draw(widget, &mut DummyWidgetState, focus, area, buffer)
                {
                    position = Some(widget_position);
                }
            });

            position
        },
        |content, state, event| {
            let mut handled = false;

            content.all(&mut state.list_content_state, &mut |widget, focus| {
                if focus == Focus::Focused {
                    handled = handle_key_event(widget, &mut DummyWidgetState, event);
                }
            });

            handled
        },
    )
}

pub fn hlist<'a, S: ListContentState>(
    pass: Pass<'a>,
    content: &mut dyn ListContent<State = S>,
) -> PassReturn<'a, State<S>> {
    list(pass, Direction::Horizontal, content)
}

pub fn vlist<'a, S: ListContentState>(
    pass: Pass<'a>,
    content: &mut dyn ListContent<State = S>,
) -> PassReturn<'a, State<S>> {
    list(pass, Direction::Vertical, content)
}
