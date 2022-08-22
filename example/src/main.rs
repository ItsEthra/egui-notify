use std::sync::Once;

use eframe::{App, egui::{Context, Style, Window}, Frame, NativeOptions};
use egui_notify::{Toasts, Toast};

#[derive(Default)]
struct ExampleApp {
    toasts: Toasts,
    caption: String,
    closable: bool,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            self.toasts.add(Toast::info("Some info with very loooooong caption"));
            self.toasts.add(Toast::warning("Some warning"));
            self.toasts.add(Toast::error("Some error"));
        });


        Window::new("Controls")
            .show(ctx, |ui| {
                ui.text_edit_singleline(&mut self.caption);
                ui.checkbox(&mut self.closable, "Closable");

                let cb = |t: Toast| t.closable(self.closable);

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
            caption: "Hello!".into(),
            ..Default::default()
        })
    }));
}
