use ratatui::layout::Constraint;

use crate::{Focus, Focusable, Pass, PassReturn, WidgetState, draw, handle_key_event, init};

pub trait ListContentState: 'static {
    fn reset_focus(&mut self) -> Focusable;
}

pub struct DummyWidgetState;

impl WidgetState for DummyWidgetState {
    fn reset_focus(&mut self) -> Focusable {
        // Using widgets and widget state here is mostly a somewhat hacky convenience thing.
        // Resetting focus on list content does not go through this function.
        unreachable!()
    }
}

pub trait ListContent {
    type State: ListContentState;

    fn init(&mut self) -> Self::State;

    fn next_constraint(&mut self) -> Option<Constraint>;

    fn all(
        &mut self,
        state: &mut Self::State,
        callback: &mut dyn FnMut(
            &mut dyn for<'a> FnMut(Pass<'a>) -> PassReturn<'a, DummyWidgetState>,
            Focus,
        ),
    );
}

pub struct ConstraintsIter<'a, S>(pub &'a mut dyn ListContent<State = S>);

impl<'a, S: ListContentState> Iterator for ConstraintsIter<'a, S> {
    type Item = Constraint;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_constraint()
    }
}

pub struct SingleWidget<F>(F, Option<Constraint>);

pub struct SingleWidgetState<S>(S);

impl<S: WidgetState> ListContentState for SingleWidgetState<S> {
    fn reset_focus(&mut self) -> Focusable {
        self.0.reset_focus()
    }
}

impl<S: WidgetState, F: for<'a> FnMut(Pass<'a>) -> PassReturn<'a, S>> ListContent
    for SingleWidget<F>
{
    type State = SingleWidgetState<S>;

    fn init(&mut self) -> Self::State {
        SingleWidgetState(init(&mut self.0))
    }

    fn next_constraint(&mut self) -> Option<Constraint> {
        self.1.take()
    }

    fn all(
        &mut self,
        state: &mut Self::State,
        callback: &mut dyn FnMut(
            &mut dyn for<'a> FnMut(Pass<'a>) -> PassReturn<'a, DummyWidgetState>,
            Focus,
        ),
    ) {
        callback(
            &mut |pass| {
                pass.apply(
                    (&mut self.0, &mut *state),
                    |_| DummyWidgetState,
                    |(widget, state), _, focus, area, buffer| {
                        draw(widget, &mut state.0, focus, area, buffer)
                    },
                    |(widget, state), _, event| handle_key_event(widget, &mut state.0, event),
                )
            },
            Focus::Focused,
        );
    }
}

pub fn fill<S: WidgetState, F: for<'a> FnMut(Pass<'a>) -> PassReturn<'a, S>>(
    fraction: u16,
    widget: F,
) -> SingleWidget<F> {
    SingleWidget(widget, Some(Constraint::Fill(fraction)))
}

pub struct SliceListContent<'a, T, W> {
    constraint: Constraint,
    slice: &'a [T],
    widget: W,
    current: usize,
}

pub struct SliceListContentState<S> {
    inner_states: Vec<S>,
    focus: usize,
}

impl<S: WidgetState> ListContentState for SliceListContentState<S> {
    fn reset_focus(&mut self) -> Focusable {
        for (i, state) in self.inner_states.iter_mut().enumerate() {
            if state.reset_focus() == Focusable::Yes {
                self.focus = i;

                return Focusable::Yes;
            }
        }

        return Focusable::No;
    }
}

impl<'a, T, S: WidgetState, W: for<'b> FnMut(Pass<'b>, &'a T) -> PassReturn<'b, S>> ListContent
    for SliceListContent<'a, T, W>
{
    type State = SliceListContentState<S>;

    fn init(&mut self) -> Self::State {
        SliceListContentState {
            inner_states: Vec::new(),
            focus: 0,
        }
    }

    fn next_constraint(&mut self) -> Option<Constraint> {
        self.current += 1;

        if self.current <= self.slice.len() {
            Some(self.constraint)
        } else {
            None
        }
    }

    fn all(
        &mut self,
        state: &mut Self::State,
        callback: &mut dyn FnMut(
            &mut dyn for<'b> FnMut(Pass<'b>) -> PassReturn<'b, DummyWidgetState>,
            Focus,
        ),
    ) {
        for (i, item) in self.slice.iter().enumerate() {
            if state.inner_states.len() <= i {
                state
                    .inner_states
                    .push(init(&mut |p| (self.widget)(p, item)));
            }

            let widget_state = &mut state.inner_states[i];

            callback(
                &mut |pass| {
                    pass.apply(
                        (&mut self.widget, &mut *widget_state),
                        |_| DummyWidgetState,
                        |(widget, state), _, focus, area, buffer| {
                            draw(&mut |p| widget(p, item), state, focus, area, buffer)
                        },
                        |(widget, state), _, event| {
                            handle_key_event(&mut |p| widget(p, item), state, event)
                        },
                    )
                },
                if state.focus == i {
                    Focus::Focused
                } else {
                    Focus::Unfocused
                },
            );
        }

        state.inner_states.truncate(self.slice.len());
    }
}

pub fn slice<'a, T, S: 'static, W: for<'b> FnMut(Pass<'b>, &'a T) -> PassReturn<'b, S>>(
    constraint: Constraint,
    slice: &'a [T],
    widget: W,
) -> SliceListContent<'a, T, W> {
    SliceListContent {
        constraint,
        slice,
        widget,
        current: 0,
    }
}

pub struct TupleListContentState<C> {
    content: C,
    focus: usize,
}

macro_rules! impl_for_tuples {
    ($($type:ident: $field:tt),*) => {
        impl<$($type: ListContentState),*> ListContentState for TupleListContentState<($($type,)*)> {
            fn reset_focus(&mut self) -> Focusable {
                $(
                    if (self.content.$field.reset_focus() == Focusable::Yes) {
                        self.focus = $field;

                        return Focusable::Yes;
                    }
                )*

                Focusable::No
            }
        }


        impl<$($type: ListContent),*> ListContent for ($($type,)*) {
            type State = TupleListContentState<($($type::State,)*)>;

            #[allow(unused_variables)]
            fn init(
                &mut self,
            ) -> Self::State {
                TupleListContentState {
                    content: ($(
                        self.$field.init(),
                    )*),
                    focus: 0,
                }
            }

            fn next_constraint(&mut self) -> Option<Constraint> {
                $(
                    if let Some(constraint) = self.$field.next_constraint() {
                        return Some(constraint);
                    }
                )*
                None
            }

            #[allow(unused_variables)]
            fn all(
                &mut self,
                state: &mut Self::State,
                callback: &mut dyn FnMut(&mut dyn for<'a> FnMut(Pass<'a>) -> PassReturn<'a, DummyWidgetState>, Focus),
            ) {
                $(
                    self.$field.all(&mut state.content.$field, &mut |widget, focus| {
                        callback(
                            widget,
                            if state.focus == $field {
                                focus
                            } else {
                                Focus::Unfocused
                            },
                        );
                    });
                )*
            }
        }
    };
}

impl_for_tuples!();
impl_for_tuples!(A: 0);
impl_for_tuples!(A: 0, B: 1);
impl_for_tuples!(A: 0, B: 1, C: 2);
impl_for_tuples!(A: 0, B: 1, C: 2, D: 3);
impl_for_tuples!(A: 0, B: 1, C: 2, D: 3, E: 4);
impl_for_tuples!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5);
impl_for_tuples!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6);
impl_for_tuples!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7);
impl_for_tuples!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8);
impl_for_tuples!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9);
impl_for_tuples!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10);
impl_for_tuples!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11);
