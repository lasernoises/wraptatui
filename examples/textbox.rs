use tui_input::Input;
use wraptatui::{
    run,
    widgets::{state::state, textbox::textbox},
};

fn main() {
    run(&mut |p| state(p, |p, input: &mut Input| textbox(p, input))).unwrap();
}
