use gpui::VisualContext;
use gpui::{
    div, rgb, DefiniteLength, FocusHandle, InteractiveElement, IntoElement, KeyDownEvent,
    ParentElement, Render, Styled, ViewContext,
};

use crate::button::*;
use crate::consts::*;
use crate::display::*;
use crate::logic::*;
use crate::styles::*;

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

    fn get_buttons(&self, cx: &mut ViewContext<Self>) -> Vec<Button> {
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

        buttons
    }
}

impl Render for Root {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let display_value = self.logic.get_display_value();
        let buttons = self.get_buttons(cx);

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
            .text_lg()
            .child(cx.new_view(|_cx| Display::new(display_value)))
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
