use eframe::{App, egui::{Context, Style, Window, Slider}, Frame, NativeOptions};
use egui_notify::{Toasts, Toast, Anchor};

struct ExampleApp {
    toasts: Toasts,
    caption: String,
    closable: bool,
    duration: f32,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        Window::new("Controls")
            .show(ctx, |ui| {
                ui.text_edit_singleline(&mut self.caption);
                ui.horizontal(|ui| {
                    ui.label("Duration (in s)");
                    ui.add(Slider::new(&mut self.duration, 1.0..=10.0));
                });
                ui.checkbox(&mut self.closable, "Closable");

                let cb = |t: Toast| t.closable(self.closable).with_duration(self.duration);

                ui.horizontal(|ui| {
                    if ui.button("Info").clicked() {
                        self.toasts.info(self.caption.clone(), cb);
                    }
    
                    if ui.button("Warning").clicked() {
                        self.toasts.warning(self.caption.clone(), cb);
                    }
    
                    if ui.button("Error").clicked() {
                        self.toasts.error(self.caption.clone(), cb);
                    }
                });
            });

        self.toasts.show(ctx);
    }
}

fn main() {
    eframe::run_native("example", NativeOptions::default(), Box::new(|cc| {
        cc.egui_ctx.set_style(Style::default());

        Box::new(ExampleApp {
            caption: "Hello! It's caption".into(),
            toasts: Toasts::default()
                .with_anchor(Anchor::TopRight),
            closable: true,
            duration: 3.5,
        })
    }));
}
