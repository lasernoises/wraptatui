# Wraptatui

Wraptatui is a [Ratatui](https://ratatui.rs/) wrapper. It's an experiment around Rust GUI
development APIs that uses some a bit of a hack to get around some Rust type system limitations.

## Example

```rust
use crossterm::event::KeyCode;
use ratatui::layout::Constraint;
use wraptatui::{
    list_content::{fill, slice},
    ratatui_widget, run,
    widgets::{list::vlist, state::state_with_default, with_key_handler::with_key_handler},
};

fn main() {
    let list = ["a", "b", "c"];

    run(&mut |p| {
        vlist(
            p,
            &mut (
                fill(1, |p| ratatui_widget(p, "Hello, World!")),
                slice(Constraint::Length(1), &list, |p, x| ratatui_widget(p, *x)),
                fill(2, |p| {
                    state_with_default(p, |p, count: &mut i32| {
                        with_key_handler(
                            p,
                            count,
                            |count, event| match event.code {
                                KeyCode::Up => {
                                    *count += 1;
                                    true
                                }
                                KeyCode::Down => {
                                    *count -= 1;
                                    true
                                }
                                _ => false,
                            },
                            |p, count| ratatui_widget(p, count.to_string()),
                        )
                    })
                }),
            ),
        )
    })
    .unwrap();
}
```

## Design

I've written about the problems I've encountered in my earlier explorations of Rust GUI libaries
[here](https://github.com/lasernoises/egrikor?tab=readme-ov-file#problems).

A lot of the problems occur when you want to return something like `impl Widget`.
In particular I want there to be an associated `State` type for each widget type.
The state is persistent, while the widget gets produced again whenever it's needed.
The widget can contain references with arbitrary lifetimes while the state is `'static`.

The idea is that producing the widget again and again should be cheap enough that it isn't a
problem.
A part of that is to generally avoid allocations in widgets.
Instead iterator like abstractions are used.
(In this particular case that is the `ListContent` trait
[here](https://github.com/lasernoises/wraptatui/blob/main/src/list_content.rs).)

The insight this particual implementation builds upon is that we don't actually need to return
something as the widget.
Instead we can just pass in an type that represents the operation we want to do on the widget and
the function can return the result.

In wraptatui a custom widget has a signature that looks something like this:

```rust
pub fn my_widget<'a>(
    pass: Pass<'a>,
    some_parameter: u32,
) -> PassReturn<'a, impl WidgetState + use<>>;
```

You can either pass on the `Pass` to another widget or handle the operation directly by calling
`apply` on it.
Both will move it and give you a `PassReturn`.
The lifetime parameter ensures that you can't return a `PassReturn` that originated from a different
`Pass`.

But now we need to resort to trickery to get the associated state type to work.
The problem is that if it gets called the second time, we need a refernce to the existing state in
`Pass`.
But `Pass` does not have a type parameter for it.
That is because Rust doesn't have a way to say that a type that occurs int the parameters of a
function should have a type that is determined by the implementation of the function.
We can only do that for returns.
`impl` in paramters means the type is determined by the caller, not the body of the function.
And even if that was possible we would still need a way to say that the that type in the parameter
and in the return is the same one.
(The
[`type_alias_impl_trait`](https://doc.rust-lang.org/beta/unstable-book/language-features/type-alias-impl-trait.html)
unstable feature makes this possible, but I wanted to avoid relying on unstable features here.)

Instead `Pass` contains an `&mut dyn Any` in this case.
The idea is that the lifetime forces you to to only be able to use the right pass with the right
return, but there is one way to use the pass for the the wrong widget.
And that is if you pass the `Pass` to the wrong widget and then panic.
This isn't really a serious concern, because it just means you will crash slightly earlier than
intended, but this is the reason we have to use a `dyn Any` with `downcast_mut` instead of being
able do use raw pointers and `unsafe`.

Of course this design is a bit ugly and impractical to use.
It's mostly just a way of working around the current limitations of Rust and seeing where we can get
with that.
