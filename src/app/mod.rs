use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use gpui::{Action, ClickEvent, Corner, ElementId, Svg, UpdateGlobal, rgb};
use gpui::{
    App, Application, Bounds, Context, Entity, Global, InteractiveElement, Render, SharedString,
    Window, WindowBounds, WindowOptions, div, prelude::*,
};
use ui::{Button, ButtonCommon, Clickable, Color, ContextMenu, DropdownMenu, Label, LabelCommon};

use crate::app::components::record_button::RecordButton;
use crate::app::state::State;
use crate::audio::engine::AudioEngine;

mod components;
mod state;

struct Root;

impl Root {
    fn new(cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for Root {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .w_full()
            .h_full()
            .flex_col()
            .gap_8()
            .items_center()
            .justify_center()
            .child(Label::new("hello").color(Color::Muted))
            .child(
                DropdownMenu::new(
                    "select",
                    "select me",
                    ContextMenu::build(window, cx, |menu, _, _| {
                        menu.entry("option", None, |win, cx| println!("selected"))
                    }),
                )
                .style(ui::DropdownStyle::Subtle),
            )
    }
}

pub fn run_app() {
    Application::new().run(|cx: &mut App| {
        let audio_engine = cx.new(|_cx| AudioEngine::new());

        cx.set_global(State::new());

        settings::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);

        cx.open_window(WindowOptions::default(), |_window, cx| cx.new(Root::new))
            .unwrap();

        cx.activate(true);
    });
}
