use eframe::egui::FontDefinitions;
use eframe::{
    egui::{Context, Slider, Window},
    App, Frame, NativeOptions,
};
use egui::{Style, Visuals};
use egui_notify::{Toast, Toasts};
use std::time::Duration;

struct ExampleApp {
    toasts: Toasts,
    caption: String,
    closable: bool,
    show_progress_bar: bool,
    expires: bool,
    duration: f32,
    dark: bool,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        Window::new("Controls").show(ctx, |ui| {
            ui.text_edit_multiline(&mut self.caption);
            ui.checkbox(&mut self.expires, "Expires");
            ui.checkbox(&mut self.closable, "Closable");
            ui.checkbox(&mut self.show_progress_bar, "ShowProgressBar");
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
                t.set_closable(self.closable)
                    .set_duration(duration)
                    .set_show_progress_bar(self.show_progress_bar);
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

            ui.separator();

            if ui.radio(self.dark, "Toggle dark theme").clicked() {
                self.dark = !self.dark;

                let mut style = ctx.style().as_ref().clone();
                if self.dark {
                    style.visuals = Visuals::dark();
                } else {
                    style.visuals = Visuals::light();
                }
                ctx.set_style(style);
            }
        });

        self.toasts.show(ctx);
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "example",
        NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_style(Style::default());

            let mut font_def = FontDefinitions::default();
            for data in font_def.font_data.values_mut() {
                data.tweak.scale = 1.25;
            }
            cc.egui_ctx.set_fonts(font_def);

            Box::new(ExampleApp {
                caption: r#"Hello! It's a multiline caption
Next line
Another one
And another one"#
                    .into(),
                toasts: Toasts::default(),
                closable: true,
                expires: true,
                show_progress_bar: true,
                duration: 3.5,
                dark: true,
            })
        }),
    )
}
