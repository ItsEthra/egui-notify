use std::sync::Once;

use eframe::{App, egui::{Context, Style, Window}, Frame, NativeOptions};
use egui_notify::{Toasts, Toast};

#[derive(Default)]
struct ExampleApp {
    toasts: Toasts
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            self.toasts.add(Toast::info("Some info"));
            self.toasts.add(Toast::warning("Some warning"));
            self.toasts.add(Toast::error("Some error"));
        });


        Window::new("Controls")
            .show(ctx, |ui| {
                let _ = ui.button("Info");
                let _ = ui.button("Warning");
                let _ = ui.button("Error");
            });

        self.toasts.show(ctx);
    }
}

fn main() {
    eframe::run_native("example", NativeOptions::default(), Box::new(|cc| {
        cc.egui_ctx.set_style(Style::default());

        Box::new(ExampleApp::default())
    }));
}
