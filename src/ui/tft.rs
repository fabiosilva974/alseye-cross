//! Tela 3: TFT SCREEN (Configurações da tela LCD e preview em tempo real)

use egui::{Color32, Pos2, Stroke, Ui, Vec2};
use crate::config::AppConfig;
use crate::hardware::HardwareMetrics;

pub struct TftUiState {
    pub selected_subtab: u8, // 0 = CPU, 1 = GPU
}

impl Default for TftUiState {
    fn default() -> Self {
        Self { selected_subtab: 0 }
    }
}

pub fn render(
    ui: &mut Ui,
    config: &mut AppConfig,
    state: &mut TftUiState,
    metrics: &HardwareMetrics,
) {
    let card_bg = Color32::from_rgb(26, 30, 46);
    let border_color = Color32::from_rgb(45, 52, 78);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 24.0;

        // Card Esquerdo: Display Settings
        egui::Frame::none()
            .fill(card_bg)
            .stroke(Stroke::new(1.0_f32, border_color))
            .rounding(10.0)
            .inner_margin(24.0)
            .show(ui, |ui| {
                ui.set_width(330.0);
                ui.set_height(420.0);

                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("Display").color(Color32::WHITE).size(18.0));
                    ui.add_space(12.0);

                    // Sub-tabs [CPU] [GPU]
                    ui.horizontal(|ui| {
                        let btn_cpu = if state.selected_subtab == 0 {
                            egui::Button::new(egui::RichText::new("CPU").color(Color32::WHITE)).fill(Color32::from_rgb(45, 60, 95))
                        } else {
                            egui::Button::new(egui::RichText::new("CPU").color(Color32::GRAY)).fill(Color32::from_rgb(30, 35, 50))
                        };
                        if ui.add(btn_cpu).clicked() {
                            state.selected_subtab = 0;
                        }

                        let btn_gpu = if state.selected_subtab == 1 {
                            egui::Button::new(egui::RichText::new("GPU").color(Color32::WHITE)).fill(Color32::from_rgb(45, 60, 95))
                        } else {
                            egui::Button::new(egui::RichText::new("GPU").color(Color32::GRAY)).fill(Color32::from_rgb(30, 35, 50))
                        };
                        if ui.add(btn_gpu).clicked() {
                            state.selected_subtab = 1;
                        }
                    });

                    ui.add_space(20.0);

                    // Toggles da aba selecionada
                    if state.selected_subtab == 0 {
                        render_toggle_row(ui, "CPU Temperature", &mut config.tft_ctemp);
                        render_toggle_row(ui, "CPU Frequency", &mut config.tft_cfreq);
                        render_toggle_row(ui, "CPU Fan", &mut config.tft_ctrpm);
                        render_toggle_row(ui, "CPU Power", &mut config.tft_cpower);
                    } else {
                        render_toggle_row(ui, "GPU Temperature", &mut config.tft_gtemp);
                        render_toggle_row(ui, "GPU Frequency", &mut config.tft_gfreq);
                        render_toggle_row(ui, "GPU Fan", &mut config.tft_gtrpm);
                        render_toggle_row(ui, "GPU Power", &mut config.tft_gpower);
                    }

                    ui.add_space(40.0);
                    render_toggle_row(ui, "On/Off", &mut config.tft_open);
                });
            });

        // Card Direito: Preview
        egui::Frame::none()
            .fill(card_bg)
            .stroke(Stroke::new(1.0_f32, border_color))
            .rounding(10.0)
            .inner_margin(24.0)
            .show(ui, |ui| {
                ui.set_width(330.0);
                ui.set_height(420.0);

                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("Preview").color(Color32::WHITE).size(18.0));
                    ui.add_space(20.0);

                    // Tela preta do LCD TFT
                    let (preview_rect, _) = ui.allocate_exact_size(Vec2::new(280.0, 160.0), egui::Sense::hover());
                    let painter = ui.painter();
                    painter.rect_filled(preview_rect, 6.0, Color32::from_rgb(10, 10, 14));

                    if config.tft_open {
                        let center = preview_rect.center();
                        let temp_val = if state.selected_subtab == 0 { metrics.cpu_temp } else { metrics.gpu_temp };
                        let label = if state.selected_subtab == 0 { "CPU" } else { "GPU" };

                        painter.text(
                            Pos2::new(center.x - 20.0, center.y - 10.0),
                            egui::Align2::CENTER_CENTER,
                            format!("{:.0}", temp_val),
                            egui::FontId::proportional(46.0),
                            Color32::WHITE,
                        );

                        painter.text(
                            Pos2::new(center.x + 45.0, center.y - 25.0),
                            egui::Align2::LEFT_CENTER,
                            "°C",
                            egui::FontId::proportional(22.0),
                            Color32::WHITE,
                        );

                        painter.text(
                            Pos2::new(center.x + 45.0, center.y + 12.0),
                            egui::Align2::LEFT_CENTER,
                            label,
                            egui::FontId::proportional(18.0),
                            Color32::WHITE,
                        );
                    } else {
                        let center = preview_rect.center();
                        painter.text(
                            center,
                            egui::Align2::CENTER_CENTER,
                            "OFF",
                            egui::FontId::proportional(24.0),
                            Color32::DARK_GRAY,
                        );
                    }

                    ui.add_space(30.0);

                    // Controles de Speed e Rotação
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Speed").color(Color32::from_rgb(180, 190, 215)).size(14.0));
                        ui.add_space(20.0);

                        for i in 0..3 {
                            let is_active = i <= config.tft_speed;
                            let bar_color = if is_active { Color32::from_rgb(52, 178, 255) } else { Color32::from_rgb(45, 52, 78) };
                            let (rect, resp) = ui.allocate_exact_size(Vec2::new(26.0, 10.0), egui::Sense::click());
                            ui.painter().rect_filled(rect, 2.0, bar_color);
                            if resp.clicked() {
                                config.tft_speed = i;
                            }
                            ui.add_space(4.0);
                        }
                    });

                    ui.add_space(16.0);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Rotation:").color(Color32::from_rgb(180, 190, 215)).size(14.0));
                        let deg = match config.tft_turn {
                            1 => "90°",
                            2 => "180°",
                            3 => "270°",
                            _ => "0°",
                        };
                        if ui.button(egui::RichText::new(deg).color(Color32::WHITE)).clicked() {
                            config.tft_turn = (config.tft_turn + 1) % 4;
                        }
                    });
                });
            });
    });
}

fn render_toggle_row(ui: &mut Ui, label: &str, value: &mut bool) {
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).color(Color32::from_rgb(180, 190, 215)).size(14.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(42.0, 22.0), egui::Sense::click());
            if resp.clicked() {
                *value = !*value;
            }

            let bg_color = if *value { Color32::from_rgb(52, 178, 255) } else { Color32::from_rgb(50, 56, 80) };
            ui.painter().rect_filled(rect, 11.0, bg_color);

            let circle_x = if *value { rect.max.x - 11.0 } else { rect.min.x + 11.0 };
            ui.painter().circle_filled(Pos2::new(circle_x, rect.center().y), 8.0, Color32::WHITE);
        });
    });
}

