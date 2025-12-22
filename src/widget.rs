use std::{any::Any, cell::Cell, marker::PhantomData};

use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
};

#[derive(Copy, Clone, Debug)]
pub enum Focus {
    Unfocused,
    Focused,
}

enum InnerPass<'a> {
    Init(),
    Draw {
        state: &'a mut dyn Any,
        focus: Focus,
        area: Rect,
        buffer: &'a mut Buffer,
        position: &'a mut Option<Position>,
    },
    Focusable {
        state: &'a mut dyn Any,
        result: &'a mut bool,
    },
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
        draw: impl Fn(B, &mut S, Focus, Rect, &mut Buffer) -> Option<Position>,
        focusable: impl Fn(B, &mut S) -> bool,
        handle_key_event: impl Fn(B, &mut S, KeyEvent) -> bool,
    ) -> PassReturn<'a, S> {
        PassReturn(match self.0 {
            InnerPass::Init() => InnerPassReturn::Init(init(borrowed)),
            InnerPass::Draw {
                state,
                focus,
                area,
                buffer,
                position,
            } => {
                *position = draw(borrowed, state.downcast_mut().unwrap(), focus, area, buffer);
                InnerPassReturn::Other(PhantomData)
            }
            InnerPass::Focusable { state, result } => {
                *result = focusable(borrowed, state.downcast_mut().unwrap());
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
    focus: Focus,
    area: Rect,
    buffer: &mut Buffer,
) -> Option<Position> {
    let mut position = None;

    match widget(Pass(InnerPass::Draw {
        state,
        focus,
        area,
        buffer,
        position: &mut position,
    }))
    .0
    {
        InnerPassReturn::Init(_) => unreachable!(),
        InnerPassReturn::Other(_) => position,
    }
}

pub fn focusable<S: 'static, W: for<'a> FnMut(Pass<'a>) -> PassReturn<S> + ?Sized>(
    widget: &mut W,
    state: &mut S,
) -> bool {
    let mut result = false;

    match widget(Pass(InnerPass::Focusable {
        state,
        result: &mut result,
    }))
    .0
    {
        InnerPassReturn::Init(_) => unreachable!(),
        InnerPassReturn::Other(_) => result,
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
