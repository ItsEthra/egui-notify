use std::time::Duration;
use eframe::{
    egui::{Context, Slider, Style, Window},
    App, Frame, NativeOptions,
};
use egui_notify::{Anchor, Toast, Toasts};

struct ExampleApp {
    toasts: Toasts,
    caption: String,
    closable: bool,
    duration: f32,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        Window::new("Controls").show(ctx, |ui| {
            ui.text_edit_singleline(&mut self.caption);
            ui.horizontal(|ui| {
                ui.label("Duration (in s)");
                ui.add(Slider::new(&mut self.duration, 1.0..=10.0));
            });
            ui.checkbox(&mut self.closable, "Closable");

            let cb = |t: &mut Toast| {
                t.set_closable(self.closable)
                    .set_duration(Some(Duration::from_millis((1000. * self.duration) as u64)));
            };

            ui.horizontal(|ui| {
                if ui.button("Success").clicked() {
                    cb(self.toasts.success(self.caption.clone()));
                }

                if ui.button("Info").clicked() {
                    cb(self.toasts.info(self.caption.clone()));
                }

                if ui.button("Warning").clicked() {
                    cb(self.toasts.warning(self.caption.clone()));
                }

                if ui.button("Error").clicked() {
                    cb(self.toasts.error(self.caption.clone()));
                }

                if ui.button("Basic").clicked() {
                    cb(self.toasts.basic(self.caption.clone()));
                }
            });
        });

        self.toasts.show(ctx);
    }
}

fn main() {
    eframe::run_native(
        "example",
        NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_style(Style::default());

            Box::new(ExampleApp {
                caption: "Hello! It's caption".into(),
                toasts: Toasts::default().with_anchor(Anchor::TopRight),
                closable: true,
                duration: 3.5,
            })
        }),
    );
}
