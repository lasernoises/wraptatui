use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
};

use crate::{Pass, PassReturn, draw, init};

pub trait ListContent {
    type State: 'static;

    fn init(&mut self) -> Self::State;

    fn next_constraint(&mut self) -> Option<Constraint>;

    fn all(
        &mut self,
        state: &mut Self::State,
        callback: &mut dyn FnMut(&mut dyn for<'a> FnMut(Pass<'a>) -> PassReturn<'a, ()>, bool),
    );
}

pub struct SingleWidget<F>(F, Option<Constraint>, bool);

impl<F> SingleWidget<F> {
    pub fn focused(mut self) -> Self {
        self.2 = true;
        self
    }

    pub fn with_focus(mut self, focus: bool) -> Self {
        self.2 = focus;
        self
    }
}

impl<S: 'static, F: for<'a> FnMut(Pass<'a>) -> PassReturn<'a, S>> ListContent for SingleWidget<F> {
    type State = S;

    fn init(&mut self) -> Self::State {
        init(&mut self.0)
    }

    fn next_constraint(&mut self) -> Option<Constraint> {
        self.1.take()
    }

    fn all(
        &mut self,
        state: &mut Self::State,
        callback: &mut dyn FnMut(&mut dyn for<'a> FnMut(Pass<'a>) -> PassReturn<'a, ()>, bool),
    ) {
        callback(
            &mut |pass| {
                pass.apply(
                    (&mut self.0, &mut *state),
                    |_: (&mut F, &mut S)| (),
                    |(widget, state): (&mut F, &mut S),
                     _: &mut (),
                     area: Rect,
                     buffer: &mut Buffer| { draw(widget, state, area, buffer) },
                    |_, _, _| false,
                )
            },
            self.2,
        );
    }
}

pub struct ConstraintsIter<'a, S>(pub &'a mut dyn ListContent<State = S>);

impl<'a, S: 'static> Iterator for ConstraintsIter<'a, S> {
    type Item = Constraint;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_constraint()
    }
}

pub fn fill<S: 'static, F: for<'a> FnMut(Pass<'a>) -> PassReturn<'a, S>>(
    fraction: u16,
    widget: F,
) -> SingleWidget<F> {
    SingleWidget(widget, Some(Constraint::Fill(fraction)), false)
}

macro_rules! impl_for_tuples {
    ($($type:ident: $field:tt),*) => {
        impl<$($type: ListContent),*> ListContent for ($($type,)*) {
            type State = ($($type::State,)*);

            #[allow(unused_variables)]
            fn init(
                &mut self,
            ) -> Self::State {
                ($(
                    self.$field.init(),
                )*)
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
                callback: &mut dyn FnMut(&mut dyn for<'a> FnMut(Pass<'a>) -> PassReturn<'a, ()>, bool),
            ) {
                $(
                    self.$field.all(&mut state.$field, callback);
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
