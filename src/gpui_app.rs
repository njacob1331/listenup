use gpui::{
    App, Application, Context, Entity, Render, SharedString, Subscription, TitlebarOptions,
    WindowOptions, div, prelude::*,
};
use gpui_component::{
    ActiveTheme as _, Disableable,
    button::{Button, ButtonVariants},
    green_500,
};

use crate::{
    core::EngineState,
    models::{AudioManager, Model},
};

mod core;
mod models;

//
// ─────────────────────────────────────────────────────────────
//   ROOT VIEW
// ─────────────────────────────────────────────────────────────
//

struct Root {
    record_button: Entity<RecordButton>,
    display: Entity<Display>,
}

impl Render for Root {
    fn render(&mut self, _win: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(cx.theme().primary)
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .items_center()
            .justify_center()
            .child(self.record_button.clone())
            .child(self.display.clone())
    }
}

//
// ─────────────────────────────────────────────────────────────
//   COMPONENTS
// ─────────────────────────────────────────────────────────────
//

struct Display {
    audio_manager: Entity<AudioManager>,
    _sub: Subscription,
}

impl Display {
    fn new(cx: &mut Context<Self>, audio_manager: &Entity<AudioManager>) -> Self {
        // Re-render the button whenever the service notifies.
        let sub = cx.observe(audio_manager, |_this, _svc, cx| cx.notify());
        Self {
            audio_manager: audio_manager.clone(),
            _sub: sub,
        }
    }
}

impl Render for Display {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let audio_manager = self.audio_manager.read(cx);

        div()
            .text_color(green_500())
            .flex()
            .flex_col()
            .gap_2()
            .child(format!("recording: {:?}", audio_manager.recording()))
            .child(format!("num samples: {}", audio_manager.samples().len()))
    }
}

struct RecordButton {
    audio_manager: Entity<AudioManager>,
    _sub: Subscription,
}

impl RecordButton {
    fn new(cx: &mut Context<Self>, audio_manager: &Entity<AudioManager>) -> Self {
        // Re-render the button whenever the service notifies.
        let sub = cx.observe(audio_manager, |_this, _svc, cx| cx.notify());
        Self {
            audio_manager: audio_manager.clone(),
            _sub: sub,
        }
    }
}

impl Render for RecordButton {
    fn render(&mut self, _win: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Derive label from service/engine state at render time.
        let state = self.audio_manager.read(cx);

        Button::new("record-btn")
            .label(format!("{:?}", state.engine_state()))
            .with_variant(gpui_component::button::ButtonVariant::Primary)
            .primary()
            .compact()
            .tooltip("tooltip")
            // // .disabled(matches!(state.engine_state(), EngineState::Recording))
            .on_click(cx.listener(|this, _ev, _win, cx| {
                this.audio_manager.update(cx, |svc, cx| {
                    svc.toggle_recording(cx);
                });
            }))
    }
}

//
// ─────────────────────────────────────────────────────────────
//   SERVICE (FACADE STORE)
// ─────────────────────────────────────────────────────────────
//

//
// ─────────────────────────────────────────────────────────────
//   MAIN
// ─────────────────────────────────────────────────────────────
//

fn main() {
    Application::new().run(|app: &mut App| {
        gpui_component::init(app);

        let audio_manager = AudioManager::init(app);

        let window = WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::new("title")),
                ..Default::default()
            }),
            ..Default::default()
        };

        let record_button = app.new(|cx| RecordButton::new(cx, &audio_manager));
        let display = app.new(|cx| Display::new(cx, &audio_manager));

        app.open_window(window, move |_, cx| {
            cx.new(|_| Root {
                record_button,
                display,
            })
        })
        .unwrap();

        app.activate(true);
    });
}
