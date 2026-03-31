mod button;
mod consts;
mod display;
mod logic;
mod root;
mod styles;

#[cfg(test)]
mod logic_test;

use gpui::{
    App, Bounds, KeyBinding, Menu, MenuItem, WindowBounds, WindowOptions, actions, application,
    prelude::*, px, size,
};
use root::*;

actions!(calculator, [Quit]);

fn main() {
    application().run(|cx: &mut App| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("alt-q", Quit, None)]);
        cx.set_menus(vec![Menu {
            name: "Calculator".into(),
            items: vec![MenuItem::action("Quit", Quit)],
            disabled: false,
        }]);
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);

        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| Root::new(cx)),
        );
    });
}
