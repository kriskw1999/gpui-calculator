mod button;
mod consts;
mod logic;
mod root;

use gpui::*;
use root::*;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);

        cx.open_window(
            WindowOptions {
                bounds: Some(bounds),
                ..Default::default()
            },
            |cx| cx.new_view(|cx| Root::new(cx)),
        );
    });
}
