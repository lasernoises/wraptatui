use crossterm::event::KeyCode;
use wraptatui::{
    list_content::fill,
    ratatui_widget, run,
    widgets::{list::vlist, state::state, with_key_handler::with_key_handler},
};

fn main() {
    run(&mut |p| {
        vlist(
            p,
            &mut (
                fill(1, |p| ratatui_widget(p, "Hello, World!")),
                fill(2, |p| {
                    state(p, &mut |p, count: &mut i32| {
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
                })
                .focused(),
            ),
        )
    })
    .unwrap();
}
