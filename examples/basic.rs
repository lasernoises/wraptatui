use wraptatui::{ratatui_widget, run};

fn main() {
    run(&mut |p| ratatui_widget(p, "Hello, World!")).unwrap();
}
