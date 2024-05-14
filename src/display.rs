use crate::styles::*;
use gpui::*;

pub struct Display {
    value: f64,
}

impl Display {
    pub fn new(value: f64) -> Self {
        Display { value }
    }
}

impl Render for Display {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .bg(rgb(DISPLAY_COLOR))
            .text_color(rgb(PRIMARY_COLOR))
            .h(DefiniteLength::Fraction(0.2))
            .px_8()
            .w_full()
            .flex()
            .items_center()
            .justify_end()
            .child(format!("{}", self.value))
    }
}
