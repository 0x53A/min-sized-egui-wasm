use egui;

pub struct AliasApp {
    is_checked: bool,
    frame_count: u64,
}

impl Default for AliasApp {
    fn default() -> Self {
        Self {
            is_checked: false,
            frame_count: 0,
        }
    }
}

impl AliasApp {
    pub fn ui(&mut self, ctx: &egui::Context) {

        // performance: keep track of frame count and render time
        self.frame_count += 1;
        let (performance, render_start_time) = {
            // Get performance object for timing
            let window = web_sys::window().expect("should have window");
            let performance = window
                .performance()
                .expect("should have performance available");
            let start_time = performance.now();
            (performance, start_time)
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            // Set dark mode
            ui.ctx().set_visuals(egui::Visuals::dark());

            ui.label("Hello World");

            ui.checkbox(&mut self.is_checked, "checkbox");

            let time = performance.now() - render_start_time;

            ui.label(format!("Frame {}: {:.2} ms", self.frame_count, time));
        });
    }
}
