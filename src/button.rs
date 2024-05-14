use crate::logic::*;
use crate::styles::*;
use gpui::*;

pub enum ButtonVariant {
    Primary,
    Secondary,
    Neutral,
}

struct ButtonStyle {
    bg: u32,
    text_color: u32,
    hover_color: u32,
}

#[derive(IntoElement)]
pub struct Button {
    on_click: Box<dyn Fn(&MouseDownEvent, &mut WindowContext) + 'static>,
    base: Div,
    text: String,
    basis: f32,
    variant: ButtonVariant,
}

impl Button {
    pub fn on_click(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_click = Box::new(handler);
        self
    }

    fn get_variant_style(&self) -> ButtonStyle {
        match self.variant {
            ButtonVariant::Neutral => ButtonStyle {
                bg: PAD_COLOR,
                text_color: WHITE_COLOR,
                hover_color: BUTTON_COLOR_HOVER,
            },
            ButtonVariant::Primary => ButtonStyle {
                bg: PRIMARY_COLOR,
                text_color: WHITE_COLOR,
                hover_color: PRIMARY_DARK,
            },
            ButtonVariant::Secondary => ButtonStyle {
                bg: BUTTON_COLOR,
                text_color: PRIMARY_COLOR,
                hover_color: BUTTON_COLOR_HOVER,
            },
        }
    }

    fn get_label(button_type: &ButtonType) -> String {
        match button_type {
            ButtonType::Reset => "C".to_owned(),
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
    pub fn new(button_type: ButtonType, basis: f32, variant: ButtonVariant) -> Self {
        Self {
            on_click: Box::new(|_event, _cx| {}),
            base: div(),
            text: Self::get_label(&button_type),
            basis,
            variant,
        }
    }
}

impl RenderOnce for Button {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let style = self.get_variant_style();

        self.base
            .w_full()
            .cursor_pointer()
            .h(DefiniteLength::Fraction(0.176))
            .bg(rgb(style.bg))
            .text_color(rgb(style.text_color))
            .hover(|this| this.bg(rgb(style.hover_color)))
            .flex_basis(DefiniteLength::Fraction(self.basis))
            .flex()
            .rounded_lg()
            .items_center()
            .justify_center()
            .child(self.text)
            .on_mouse_down(MouseButton::Left, move |event, cx| {
                (&self.on_click)(event, cx)
            })
    }
}
