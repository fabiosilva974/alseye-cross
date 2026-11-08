//! Tela 2: HARDWARE INFORMATION (Especificações e componentes do sistema)

use egui::{Color32, Stroke, Ui};
use crate::hardware::SystemInfo;

pub fn render(ui: &mut Ui, info: &SystemInfo) {
    let card_bg = Color32::from_rgb(26, 30, 46);
    let border_color = Color32::from_rgb(45, 52, 78);

    egui::Frame::none()
        .fill(card_bg)
        .stroke(Stroke::new(1.0_f32, border_color))
        .rounding(10.0)
        .inner_margin(36.0)
        .show(ui, |ui| {
            ui.set_width(700.0);
            ui.set_height(420.0);

            ui.vertical(|ui| {
                ui.add_space(10.0);

                let rows = [
                    ("CPU", &info.cpu_name),
                    ("Mainboard", &info.mainboard),
                    ("RAM", &info.ram_summary),
                    ("Disk Drive", &info.disk_name),
                    ("Graphics Card", &info.gpu_name),
                    ("Sound Card", &info.sound_card),
                    ("Network Card", &info.network_card),
                    ("System Version", &info.os_version),
                ];

                for (label, val) in rows {
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(label)
                                .color(Color32::from_rgb(180, 190, 215))
                                .size(14.0)
                        );
                        ui.add_space(40.0);
                        ui.label(
                            egui::RichText::new(val)
                                .color(Color32::WHITE)
                                .size(14.0)
                        );
                    });
                }
            });
        });
}

