use crate::logic::*;
use gpui::*;

#[derive(IntoElement)]
pub struct Button {
    on_click: Box<dyn Fn(&MouseDownEvent, &mut WindowContext) + 'static>,
    base: Div,
    text: String,
}

impl Button {
    pub fn on_click(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_click = Box::new(handler);
        self
    }

    pub fn get_label(button_type: &ButtonType) -> String {
        match button_type {
            ButtonType::Reset => "AC".to_owned(),
            ButtonType::Sign => "+/-".to_owned(),
            ButtonType::Percent => "%".to_owned(),
            ButtonType::Aritmethic(Operation::Division) => "/".to_owned(),
            ButtonType::Aritmethic(Operation::Times) => "x".to_owned(),
            ButtonType::Aritmethic(Operation::Minus) => "-".to_owned(),
            ButtonType::Aritmethic(Operation::Plus) => "+".to_owned(),
            ButtonType::Comma => ",".to_owned(),
            ButtonType::Equal => "=".to_owned(),
            ButtonType::Number(num) => format!("{}", num),
        }
    }
}

impl Button {
    pub fn new(button_type: ButtonType) -> Self {
        Self {
            on_click: Box::new(|_event, _cx| {}),
            base: div(),
            text: Self::get_label(&button_type).to_owned(),
        }
    }
}

impl RenderOnce for Button {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        self.base
            .w_full()
            .cursor_pointer()
            .h(DefiniteLength::Fraction(0.176))
            .bg(rgb(0x7f4f24))
            .text_color(rgb(0xffe6a7))
            .hover(|this| this.bg(rgb(0x936639)))
            .flex_basis(DefiniteLength::Fraction(0.225))
            .flex()
            .rounded_2xl()
            .items_center()
            .justify_center()
            .child(self.text)
            .on_mouse_down(MouseButton::Left, move |event, cx| {
                (&self.on_click)(event, cx)
            })
    }
}
