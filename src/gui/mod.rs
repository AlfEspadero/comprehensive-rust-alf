mod utils;
mod widget;
mod window;
use crate::gui::utils::{Button, Label};
use crate::gui::widget::Widget;
use crate::gui::window::Window;

pub fn gui_test() {
	let mut window = Window::new("Rust GUI Demo 1.23");
	window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
	window.add_widget(Box::new(Button::new("Click me!")));
	window.draw();
}
