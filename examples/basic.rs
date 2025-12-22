use crossterm::event::KeyCode;
use ratatui::layout::Constraint;
use wraptatui::{
    Focusable,
    list_content::{fill, slice},
    ratatui_widget, run,
    widgets::{list::vlist, state::state, with_key_handler::with_key_handler},
};

fn main() {
    let list = ["a", "b", "c"];

    run(&mut |p| {
        vlist(
            p,
            &mut (
                fill(1, |p| ratatui_widget(p, Focusable::No, "Hello, World!")),
                slice(Constraint::Length(1), &list, |p, x| {
                    ratatui_widget(p, Focusable::No, *x)
                }),
                fill(2, |p| {
                    state(p, |p, count: &mut i32| {
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
                            |p, count| ratatui_widget(p, Focusable::No, count.to_string()),
                        )
                    })
                }),
            ),
        )
    })
    .unwrap();
}
