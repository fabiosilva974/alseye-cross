//! Tela 1: HARDWARE MONITOR (Mostradores circulares e barras de progresso)

use egui::{Color32, Pos2, Rect, Stroke, Ui, Vec2};
use crate::hardware::HardwareMetrics;

pub fn render(ui: &mut Ui, metrics: &HardwareMetrics) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 24.0;

        // Card CPU
        render_metric_card(
            ui,
            "CPU",
            metrics.cpu_temp,
            &[
                ("CPU Utilization", format!("{:.0} %", metrics.cpu_usage), metrics.cpu_usage / 100.0),
                ("CPU Frequency", format!("{:.0} MHz", metrics.cpu_freq_mhz), (metrics.cpu_freq_mhz / 5000.0).clamp(0.0, 1.0)),
                ("CPU Fan", format!("{:.0} RPM", metrics.cpu_fan_rpm), (metrics.cpu_fan_rpm / 3000.0).clamp(0.0, 1.0)),
                ("CPU Power", format!("{:.0} W", metrics.cpu_power_w), (metrics.cpu_power_w / 150.0).clamp(0.0, 1.0)),
            ],
        );

        // Card GPU
        render_metric_card(
            ui,
            "GPU",
            metrics.gpu_temp,
            &[
                ("GPU Utilization", format!("{:.0} %", metrics.gpu_usage), metrics.gpu_usage / 100.0),
                ("GPU Frequency", format!("{:.0} MHz", metrics.gpu_freq_mhz), (metrics.gpu_freq_mhz / 2500.0).clamp(0.0, 1.0)),
                ("GPU Fan", format!("{:.0} RPM", metrics.gpu_fan_rpm), (metrics.gpu_fan_rpm / 3500.0).clamp(0.0, 1.0)),
                ("GPU Power", format!("{:.0} W", metrics.gpu_power_w), (metrics.gpu_power_w / 250.0).clamp(0.0, 1.0)),
            ],
        );
    });
}

fn render_metric_card(ui: &mut Ui, title: &str, temp: f32, bars: &[(&str, String, f32)]) {
    let card_bg = Color32::from_rgb(26, 30, 46);
    let border_color = Color32::from_rgb(45, 52, 78);

    egui::Frame::none()
        .fill(card_bg)
        .stroke(Stroke::new(1.0_f32, border_color))
        .rounding(10.0)
        .inner_margin(24.0)
        .show(ui, |ui| {
            ui.set_width(330.0);
            ui.set_height(420.0);

            ui.vertical_centered(|ui| {
                // Desenho do mostrador circular (Gauge)
                let (rect, _response) = ui.allocate_exact_size(Vec2::new(160.0, 160.0), egui::Sense::hover());
                let painter = ui.painter();
                let center = rect.center();
                let radius = 68.0;

                // Anel de fundo escuro
                painter.circle_stroke(center, radius, Stroke::new(9.0_f32, Color32::from_rgb(38, 44, 68)));

                // Arco de preenchimento (temperatura proporcional até 100°C)
                let pct = (temp / 100.0).clamp(0.0, 1.0);
                let start_angle = -std::f32::consts::FRAC_PI_2;
                let sweep = pct * std::f32::consts::TAU;
                let steps = 40;
                let cyan_color = Color32::from_rgb(52, 178, 255);

                for i in 0..steps {
                    let a1 = start_angle + (i as f32 / steps as f32) * sweep;
                    let a2 = start_angle + ((i + 1) as f32 / steps as f32) * sweep;
                    let p1 = Pos2::new(center.x + radius * a1.cos(), center.y + radius * a1.sin());
                    let p2 = Pos2::new(center.x + radius * a2.cos(), center.y + radius * a2.sin());
                    painter.line_segment([p1, p2], Stroke::new(9.0_f32, cyan_color));
                }

                // Texto da Temperatura no centro
                painter.text(
                    Pos2::new(center.x, center.y - 8.0),
                    egui::Align2::CENTER_CENTER,
                    format!("{:.0}°c", temp),
                    egui::FontId::proportional(32.0),
                    Color32::WHITE,
                );

                // Subtítulo (CPU / GPU)
                painter.text(
                    Pos2::new(center.x, center.y + 24.0),
                    egui::Align2::CENTER_CENTER,
                    title,
                    egui::FontId::proportional(15.0),
                    Color32::from_rgb(170, 180, 205),
                );

                ui.add_space(20.0);

                // Barras horizontais de métricas
                for (label, val_str, progress) in bars {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(*label).color(Color32::from_rgb(180, 190, 215)).size(13.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(egui::RichText::new(val_str).color(Color32::WHITE).size(13.0));
                        });
                    });

                    // Barra customizada
                    let bar_height = 8.0;
                    let (bar_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), bar_height), egui::Sense::hover());
                    ui.painter().rect_filled(bar_rect, 4.0, Color32::from_rgb(38, 44, 68));

                    let fill_w = bar_rect.width() * progress.clamp(0.0, 1.0);
                    if fill_w > 0.0 {
                        let fill_rect = Rect::from_min_size(bar_rect.min, Vec2::new(fill_w, bar_height));
                        ui.painter().rect_filled(fill_rect, 4.0, Color32::from_rgb(70, 150, 240));
                    }
                }
            });
        });
}

