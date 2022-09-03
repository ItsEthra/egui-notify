use eframe::{
    egui::{Context, Slider, Style, Window},
    App, Frame, NativeOptions,
};
use egui_notify::{Anchor, Toast, Toasts};
use std::time::Duration;

struct ExampleApp {
    toasts: Toasts,
    caption: String,
    closable: bool,
    expires: bool,
    duration: f32,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        Window::new("Controls").show(ctx, |ui| {
            ui.text_edit_singleline(&mut self.caption);
            ui.checkbox(&mut self.expires, "Expires");
            ui.checkbox(&mut self.closable, "Closable");
            if !(self.expires || self.closable) {
                ui.label("Warning; toasts will have to be closed programatically");
            }
            ui.add_enabled_ui(self.expires, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Duration (in s)");
                    ui.add(Slider::new(&mut self.duration, 1.0..=10.0));
                });
            });

            let customize_toast = |t: &mut Toast| {
                let duration = if self.expires {
                    Some(Duration::from_millis((1000. * self.duration) as u64))
                } else {
                    None
                };
                t.set_closable(self.closable).set_duration(duration);
            };

            ui.horizontal(|ui| {
                if ui.button("Success").clicked() {
                    customize_toast(self.toasts.success(self.caption.clone()));
                }

                if ui.button("Info").clicked() {
                    customize_toast(self.toasts.info(self.caption.clone()));
                }

                if ui.button("Warning").clicked() {
                    customize_toast(self.toasts.warning(self.caption.clone()));
                }

                if ui.button("Error").clicked() {
                    customize_toast(self.toasts.error(self.caption.clone()));
                }

                if ui.button("Basic").clicked() {
                    customize_toast(self.toasts.basic(self.caption.clone()));
                }
            });

            ui.separator();

            if ui.button("Dismiss all toasts").clicked() {
                self.toasts.dismiss_all_toasts();
            }
            if ui.button("Dismiss latest toast").clicked() {
                self.toasts.dismiss_latest_toast();
            }
            if ui.button("Dismiss oldest toast").clicked() {
                self.toasts.dismiss_oldest_toast();
            }
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
                expires: true,
                duration: 3.5,
            })
        }),
    );
}
