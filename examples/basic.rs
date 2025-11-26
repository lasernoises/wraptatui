use wraptatui::{list_content::fill, ratatui_widget, run, widgets::list::vlist};

fn main() {
    run(&mut |p| {
        vlist(
            p,
            &mut (
                fill(1, |p| ratatui_widget(p, "Hello, World!")),
                fill(1, |p| ratatui_widget(p, "Hello, World!")),
                fill(2, |p| ratatui_widget(p, "Hello, World!")),
            ),
        )
    })
    .unwrap();
}
