use gpui::{
    App, Application, Bounds, Context, Entity, Render, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};

pub struct Footer;

impl Render for Footer {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().bg(rgb(0xff0000)).child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .bottom_0()
                .child("some text")
                .child("some other text"),
        )
    }
}

impl Footer {
    pub fn new() -> Self {
        Self
    }
}
