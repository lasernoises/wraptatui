use std::{any::Any, cell::Cell, marker::PhantomData};

use ratatui::{buffer::Buffer, layout::Rect};

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
        init: impl Fn(B) -> S,
        draw: impl Fn(B, &mut S, Rect, &mut Buffer),
    ) -> PassReturn<'a, S> {
        PassReturn(match self.0 {
            InnerPass::Init() => InnerPassReturn::Init(init(borrowed)),
            InnerPass::Draw(state, area, buffer) => {
                draw(borrowed, state.downcast_mut().unwrap(), area, buffer);
                InnerPassReturn::Other(PhantomData)
            }
        })
    }
}

pub fn init<S: 'static, W: for<'a> FnMut(Pass<'a>) -> PassReturn<S> + ?Sized>(widget: &mut W) -> S {
    match widget(Pass(InnerPass::Init())).0 {
        InnerPassReturn::Init(state) => state,
        InnerPassReturn::Other(_) => unreachable!(),
    }
}

pub fn draw<S: 'static, W: for<'a> FnMut(Pass<'a>) -> PassReturn<S> + ?Sized>(
    widget: &mut W,
    state: &mut S,
    area: Rect,
    buffer: &mut Buffer,
) {
    match widget(Pass(InnerPass::Draw(state, area, buffer))).0 {
        InnerPassReturn::Init(_) => unreachable!(),
        InnerPassReturn::Other(_) => (),
    }
}
