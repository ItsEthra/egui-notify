use eframe::egui::FontDefinitions;
use eframe::{
    egui::{Context, Slider, Window},
    App, Frame, NativeOptions,
};
use egui::{Color32, Shadow, Style, Visuals};
use egui_notify::{Toast, Toasts};
use std::time::Duration;

struct ExampleApp {
    toasts: Toasts,
    caption: String,
    closable: bool,
    show_progress_bar: bool,
    expires: bool,
    duration: f32,
    font_size: f32,
    dark: bool,
    custom_level_string: String,
    custom_level_color: Color32,
    shadow: bool,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        Window::new("Controls").show(ctx, |ui| {
            ui.text_edit_multiline(&mut self.caption);
            ui.checkbox(&mut self.expires, "Expires");
            ui.checkbox(&mut self.closable, "Closable");
            ui.checkbox(&mut self.show_progress_bar, "ShowProgressBar");
            ui.checkbox(&mut self.shadow, "Shadow").clicked().then(|| {
                self.toasts = if self.shadow {
                    Toasts::default().with_shadow(Shadow {
                        offset: Default::default(),
                        blur: 30.0,
                        spread: 5.0,
                        color: Color32::from_black_alpha(70),
                    })
                } else {
                    Toasts::default()
                };
            });
            if !(self.expires || self.closable) {
                ui.label("Warning; toasts will have to be closed programatically");
            }
            ui.add_enabled_ui(self.expires, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Duration (in s)");
                    ui.add(Slider::new(&mut self.duration, 1.0..=10.0));
                });
                ui.horizontal(|ui| {
                    ui.label("Font size");
                    ui.add(Slider::new(&mut self.font_size, 8.0..=20.0));
                });
            });
            ui.text_edit_singleline(&mut self.custom_level_string);
            ui.color_edit_button_srgba(&mut self.custom_level_color);

            let customize_toast = |t: &mut Toast| {
                let duration = if self.expires {
                    Some(Duration::from_millis((1000. * self.duration) as u64))
                } else {
                    None
                };

                t.closable(self.closable)
                    .duration(duration)
                    .show_progress_bar(self.show_progress_bar);
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

                if ui.button("Rich text").clicked() {
                    customize_toast(
                        self.toasts.success(
                            egui::RichText::new(self.caption.clone())
                                .color(Color32::GREEN)
                                .background_color(Color32::DARK_GRAY)
                                .size(self.font_size)
                                .italics()
                                .underline(),
                        ),
                    );
                }

                if ui.button("Custom").clicked() {
                    customize_toast(self.toasts.custom(
                        self.caption.clone(),
                        self.custom_level_string.clone(),
                        self.custom_level_color,
                    ));
                }

                if ui
                    .button("Phosphor")
                    .on_hover_text("This toast uses egui-phosphor icons")
                    .clicked()
                {
                    customize_toast(self.toasts.custom(
                        self.caption.clone(),
                        egui_phosphor::regular::FAN.to_owned(),
                        self.custom_level_color,
                    ));
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
            egui_phosphor::add_to_fonts(&mut font_def, egui_phosphor::Variant::Regular);
            // for data in font_def.font_data.values_mut() {
            //     data.tweak.scale = 1.25;
            // }
            cc.egui_ctx.set_fonts(font_def);

            Ok(Box::new(ExampleApp {
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
                font_size: 16.,
                custom_level_string: "$".into(),
                custom_level_color: egui::Color32::GREEN,
                shadow: true,
            }))
        }),
    )
}
