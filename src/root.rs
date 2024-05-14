use crate::button::*;
use crate::consts::*;
use crate::logic::*;
use crate::styles::*;
use gpui::*;

pub struct Root {
    pub logic: Logic,
    focus_handle: FocusHandle,
}

impl Root {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let logic = Logic::new();

        Self {
            logic,
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for Root {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let display_value = self.logic.get_display_value();

        let mut buttons = Vec::new();

        for button_type in BUTTONS {
            let basis = match button_type {
                ButtonType::Equal => 0.47,
                _ => 0.225,
            };

            let variant = match button_type {
                ButtonType::Equal => ButtonVariant::Primary,
                ButtonType::Number(_) => ButtonVariant::Neutral,
                ButtonType::Comma => ButtonVariant::Neutral,
                _ => ButtonVariant::Secondary,
            };

            let button = Button::new(button_type, basis, variant).on_click(cx.listener(
                move |this, _view, cx| {
                    this.logic.on_button_pressed(button_type);

                    cx.notify()
                },
            ));

            buttons.push(button);
        }

        // To accept key stroke events it is necessary to focus the
        // view at the beginning
        cx.focus(&self.focus_handle);

        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, cx| {
                this.logic.handle_key_input(&event.keystroke.key.as_str());

                cx.notify();
            }))
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(PAD_COLOR))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .bg(rgb(DISPLAY_COLOR))
                    .text_color(rgb(PRIMARY_COLOR))
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
                    .children(buttons),
            )
    }
}
