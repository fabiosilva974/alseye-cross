pub mod monitor;
pub mod info;
pub mod tft;

use egui::{Color32, Stroke, Ui, Vec2};
use crate::config::AppConfig;
use crate::hardware::{HardwareMetrics, SystemInfo};
use tft::TftUiState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    HardwareMonitor,
    HardwareInformation,
    TftScreen,
}

pub struct AppUi {
    pub active_tab: ActiveTab,
    pub tft_state: TftUiState,
}

impl AppUi {
    pub fn new() -> Self {
        Self {
            active_tab: ActiveTab::HardwareMonitor,
            tft_state: TftUiState::default(),
        }
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        config: &mut AppConfig,
        metrics: &HardwareMetrics,
        info: &SystemInfo,
    ) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(20, 23, 34)).inner_margin(20.0))
            .show(ctx, |ui| {
                // Barra superior
                render_topbar(ui);
                ui.add_space(20.0);

                // Área de conteúdo: Sidebar à esquerda + Conteúdo central à direita
                ui.horizontal(|ui| {
                    // Sidebar
                    self.render_sidebar(ui);
                    ui.add_space(24.0);

                    // Conteúdo da aba selecionada
                    ui.vertical(|ui| {
                        match self.active_tab {
                            ActiveTab::HardwareMonitor => {
                                monitor::render(ui, metrics);
                            }
                            ActiveTab::HardwareInformation => {
                                info::render(ui, info);
                            }
                            ActiveTab::TftScreen => {
                                tft::render(ui, config, &mut self.tft_state, metrics);
                            }
                        }
                    });
                });

                // Rodapé com watermark
                ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                    ui.label(
                        egui::RichText::new("www.alseyecorp.com")
                            .color(Color32::from_rgb(60, 68, 95))
                            .size(11.0)
                    );
                });
            });
    }

    fn render_sidebar(&mut self, ui: &mut Ui) {
        let sidebar_width = 150.0;
        let btn_height = 54.0;

        ui.vertical(|ui| {
            ui.set_width(sidebar_width);
            ui.spacing_mut().item_spacing.y = 16.0;

            let tabs = [
                (ActiveTab::HardwareMonitor, "HARDWARE\nMONITOR"),
                (ActiveTab::HardwareInformation, "HARDWARE\nINFORMATION"),
                (ActiveTab::TftScreen, "TFT SCREEN"),
            ];

            for (tab, label) in tabs {
                let is_active = self.active_tab == tab;
                let bg_color = if is_active {
                    Color32::from_rgb(32, 38, 58)
                } else {
                    Color32::from_rgb(24, 28, 42)
                };

                let stroke = if is_active {
                    Stroke::new(1.5_f32, Color32::from_rgb(100, 130, 240))
                } else {
                    Stroke::new(1.0_f32, Color32::from_rgb(45, 52, 78))
                };

                let text_color = if is_active { Color32::WHITE } else { Color32::from_rgb(160, 170, 195) };

                let btn = egui::Button::new(
                    egui::RichText::new(label)
                        .color(text_color)
                        .size(11.0)
                        .strong()
                )
                .fill(bg_color)
                .stroke(stroke)
                .rounding(8.0)
                .min_size(Vec2::new(sidebar_width, btn_height));

                if ui.add(btn).clicked() {
                    self.active_tab = tab;
                }
            }
        });
    }
}

fn render_topbar(ui: &mut Ui) {
    ui.horizontal(|ui| {
        // Logo ALSEYE BEM
        ui.label(
            egui::RichText::new("ALSEYE")
                .color(Color32::WHITE)
                .size(22.0)
                .strong()
        );
        ui.label(
            egui::RichText::new("®  BEM")
                .color(Color32::from_rgb(180, 190, 215))
                .size(13.0)
        );

        // Botões de Janela à direita
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(egui::RichText::new("✕").color(Color32::WHITE).size(14.0)).clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
            if ui.button(egui::RichText::new("—").color(Color32::WHITE).size(14.0)).clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Minimized(true));
            }
            ui.label(egui::RichText::new("☰").color(Color32::WHITE).size(15.0));
            ui.label(egui::RichText::new("⟳").color(Color32::WHITE).size(16.0));
        });
    });
}

