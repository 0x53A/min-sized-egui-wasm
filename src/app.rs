use eframe::egui::{self, Pos2, Stroke};
use std::f32::consts::PI;

#[derive(Default)]
pub struct App {
    start_time: Option<f64>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let current_time = ctx.input(|i| i.time);

        // Initialize start time on first frame
        if self.start_time.is_none() {
            self.start_time = Some(current_time);
        }

        let delta: f32 = (current_time - self.start_time.unwrap()) as f32;

        // Calculate rotation based on time
        let rotation_period = 1.0; // seconds for a full rotation
        let angle = (delta % rotation_period) / rotation_period * 2.0 * PI;

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::BLACK))
            .show(ctx, |ui| {
                let center = ui.available_size() * 0.5;
                let size = center.x.min(center.y) * 0.25;
                let z_offset = 200.0;

                // Create vertices relative to cube's center (remove z_offset from initial coordinates)
                let vertices = [
                    [-size, -size, size], // Front face
                    [size, -size, size],
                    [size, size, size],
                    [-size, size, size],
                    [-size, -size, -size], // Back face
                    [size, -size, -size],
                    [size, size, -size],
                    [-size, size, -size],
                ];

                let edges = [
                    (0, 1),
                    (1, 2),
                    (2, 3),
                    (3, 0), // Front face
                    (4, 5),
                    (5, 6),
                    (6, 7),
                    (7, 4), // Back face
                    (0, 4),
                    (1, 5),
                    (2, 6),
                    (3, 7), // Connecting edges
                ];

                let painter = ui.painter();

                let transformed: Vec<Pos2> = vertices
                    .iter()
                    .map(|&[x, y, z]| {
                        // Y-axis rotation around cube's center
                        let x_rot = x * angle.cos() + z * angle.sin();
                        let z_rot = -x * angle.sin() + z * angle.cos();

                        // Add z_offset after rotation for perspective
                        let z_with_offset = z_rot + z_offset;

                        // Perspective projection
                        let scale = 800.0 / (z_with_offset + 800.0);
                        Pos2::new(center.x + x_rot * scale, center.y + y * scale)
                    })
                    .collect();

                for &(i, j) in edges.iter() {
                    painter.line_segment(
                        [transformed[i], transformed[j]],
                        Stroke::new(2.5, egui::Color32::from_rgb(0, 255, 128)),
                    );
                }
            });

        ctx.request_repaint();
    }
}
