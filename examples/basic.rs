use wraptatui::{
    list_content::fill,
    ratatui_widget, run,
    widgets::{list::vlist, state::state},
};

fn main() {
    run(&mut |p| {
        vlist(
            p,
            &mut (
                fill(1, |p| ratatui_widget(p, "Hello, World!")),
                fill(1, |p| ratatui_widget(p, "Hello, World!")),
                fill(2, |p| ratatui_widget(p, "Hello, World!")),
                fill(2, |p| {
                    state(p, &mut |p, text: &mut String| ratatui_widget(p, &*text))
                }),
            ),
        )
    })
    .unwrap();
}
