use std::{any::Any, cell::Cell, marker::PhantomData};

use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

enum InnerPass<'a> {
    Init(),
    Draw(&'a mut dyn Any, Rect, &'a mut Buffer),
    HandleKeyEvent(&'a mut dyn Any, KeyEvent, &'a mut bool),
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
        handle_key_event: impl Fn(B, &mut S, KeyEvent) -> bool,
    ) -> PassReturn<'a, S> {
        PassReturn(match self.0 {
            InnerPass::Init() => InnerPassReturn::Init(init(borrowed)),
            InnerPass::Draw(state, area, buffer) => {
                draw(borrowed, state.downcast_mut().unwrap(), area, buffer);
                InnerPassReturn::Other(PhantomData)
            }
            InnerPass::HandleKeyEvent(state, event, handled) => {
                *handled = handle_key_event(borrowed, state.downcast_mut().unwrap(), event);
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

pub fn handle_key_event<S: 'static, W: for<'a> FnMut(Pass<'a>) -> PassReturn<S> + ?Sized>(
    widget: &mut W,
    state: &mut S,
    event: KeyEvent,
) -> bool {
    let mut handled = false;

    match widget(Pass(InnerPass::HandleKeyEvent(state, event, &mut handled))).0 {
        InnerPassReturn::Init(_) => unreachable!(),
        InnerPassReturn::Other(_) => handled,
    }
}
