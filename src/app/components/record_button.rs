use ui::{Button, ButtonCommon, Clickable, Color, Render};

pub struct RecordButton;

impl Render for RecordButton {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        Button::new("id", "click")
            .style(ui::ButtonStyle::Filled)
            .color(Color::Default)
            .on_click(cx.listener(|this, event, win, cx| println!("i got clicked")))
    }
}
