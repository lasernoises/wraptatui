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
