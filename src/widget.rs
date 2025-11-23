use std::{any::Any, cell::Cell, marker::PhantomData};

use ratatui::{buffer::Buffer, layout::Rect};

pub struct Widget<I, D> {
    pub init: I,
    pub draw: D,
}

enum InnerPass<'a> {
    Init(),
    Draw(&'a mut dyn Any, Rect, &'a mut Buffer),
}

enum InnerPassReturn<'a, S> {
    Init(S),
    Other(PhantomData<Cell<&'a f32>>),
}

pub struct PassReturn<'a, S>(InnerPassReturn<'a, S>);

pub struct Pass<'a>(InnerPass<'a>);

impl<'a> Pass<'a> {
    pub fn apply<B, S: 'static>(
        self,
        borrowed: B,
        widget: Widget<impl Fn(B) -> S, impl Fn(B, &mut S, Rect, &mut Buffer)>,
    ) -> PassReturn<'a, S> {
        PassReturn(match self.0 {
            InnerPass::Init() => InnerPassReturn::Init((widget.init)(borrowed)),
            InnerPass::Draw(state, area, buffer) => {
                (widget.draw)(borrowed, state.downcast_mut().unwrap(), area, buffer);
                InnerPassReturn::Other(PhantomData)
            }
        })
    }
}

pub fn init<S: 'static>(mut widget: impl for<'a> FnMut(Pass<'a>) -> PassReturn<S>) -> S {
    match widget(Pass(InnerPass::Init())).0 {
        InnerPassReturn::Init(state) => state,
        InnerPassReturn::Other(_) => unreachable!(),
    }
}

pub fn draw<S: 'static>(
    mut widget: impl for<'a> FnMut(Pass<'a>) -> PassReturn<S>,
    state: &mut S,
    area: Rect,
    buffer: &mut Buffer,
) {
    match widget(Pass(InnerPass::Draw(state, area, buffer))).0 {
        InnerPassReturn::Init(_) => unreachable!(),
        InnerPassReturn::Other(_) => (),
    }
}
