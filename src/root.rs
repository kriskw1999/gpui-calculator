use crate::button::*;
use crate::consts::*;
use crate::logic::*;
use gpui::*;

pub struct Root {
    pub logic: Logic,
}

impl Root {
    pub fn new(_cx: &mut ViewContext<Self>) -> Self {
        let logic = Logic::new();

        Self { logic }
    }
}

impl Render for Root {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let display_value = self.logic.get_display_value();

        let mut buttons = Vec::new();

        for button_type in BUTTONS {
            let button = Button::new(button_type).on_click(cx.listener(move |this, _view, cx| {
                this.logic.on_button_pressed(button_type);

                cx.notify()
            }));

            buttons.push(button);
        }

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x6f1d1b))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .bg(rgb(0x432818))
                    .text_color(rgb(0xffe6a7))
                    .h(DefiniteLength::Fraction(0.2))
                    .px_8()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_end()
                    .child(format!("{}", display_value)),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .justify_center()
                    .h(DefiniteLength::Fraction(0.80))
                    .py(DefiniteLength::Fraction(0.02))
                    .gap(DefiniteLength::Fraction(0.02))
                    .children(buttons)
                    .child(
                        div()
                            .w_full()
                            .cursor_pointer()
                            .h(DefiniteLength::Fraction(0.176))
                            .flex_basis(DefiniteLength::Fraction(0.225))
                            .flex(),
                    ),
            )
    }
}
