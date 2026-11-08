//! ALSEYE BEM Multiplatform (Linux & Windows)
//! Controlador e Monitor de Hardware com suporte a Tela TFT

mod config;
mod hardware;
mod protocol;
mod ui;

use config::AppConfig;
use eframe::egui;
use hardware::{HardwareMetrics, HardwareMonitor, SystemInfo};
use protocol::{build_standard_report, build_tft_report, Rotation, TftSpeed, PRODUCT_ID, VENDOR_ID};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use ui::AppUi;

struct AlseyeApp {
    config: AppConfig,
    config_path: PathBuf,
    metrics: Arc<Mutex<HardwareMetrics>>,
    info: SystemInfo,
    ui: AppUi,
}

impl AlseyeApp {
    fn new(
        _cc: &eframe::CreationContext<'_>,
        config: AppConfig,
        config_path: PathBuf,
        metrics: Arc<Mutex<HardwareMetrics>>,
        info: SystemInfo,
    ) -> Self {
        Self {
            config,
            config_path,
            metrics,
            info,
            ui: AppUi::new(),
        }
    }
}

impl eframe::App for AlseyeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Solicita repintura suave a cada 1 segundo
        ctx.request_repaint_after(Duration::from_millis(1000));

        let current_metrics = {
            let m = self.metrics.lock().unwrap();
            m.clone()
        };

        self.ui.render(ctx, &mut self.config, &current_metrics, &self.info);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.config.save(&self.config_path);
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();

    let config_path = PathBuf::from("config.json");
    let config = AppConfig::load_or_default(&config_path);
    let config_arc = Arc::new(Mutex::new(config.clone()));

    let mut hw = HardwareMonitor::new();
    hw.refresh();
    let sys_info = hw.info.clone();

    let metrics_arc = Arc::new(Mutex::new(hw.metrics.clone()));
    let metrics_clone = Arc::clone(&metrics_arc);
    let config_thread = Arc::clone(&config_arc);

    // Thread em background: Lê sensores e transmite os pacotes USB HID
    thread::spawn(move || {
        let mut hid_device: Option<hidapi::HidDevice> = None;
        let mut api_opt: Option<hidapi::HidApi> = hidapi::HidApi::new().ok();

        loop {
            // Atualiza leituras de hardware
            hw.refresh();
            {
                let mut m = metrics_clone.lock().unwrap();
                *m = hw.metrics.clone();
            }

            let (tft_open, speed_idx, turn_idx) = {
                let cfg = config_thread.lock().unwrap();
                (cfg.tft_open, cfg.tft_speed, cfg.tft_turn)
            };

            let cpu_temp = hw.metrics.cpu_temp.round().clamp(0.0, 120.0) as u8;
            let gpu_temp = hw.metrics.gpu_temp.round().clamp(0.0, 120.0) as u8;
            let rpm = hw.metrics.cpu_fan_rpm.round().clamp(0.0, 5000.0) as u16;

            // Gerenciamento de conexão USB HID
            if let Some(ref api) = api_opt {
                if hid_device.is_none() {
                    if let Ok(dev) = api.open(VENDOR_ID, PRODUCT_ID) {
                        println!("[+] Watercooler ALSEYE BEM conectado via USB HID!");
                        hid_device = Some(dev);
                    }
                }

                if let Some(ref dev) = hid_device {
                    let packet = if tft_open {
                        let rot = Rotation::from_index(turn_idx);
                        build_tft_report(rot, cpu_temp, gpu_temp, rpm)
                    } else {
                        build_standard_report(cpu_temp)
                    };

                    if let Err(e) = dev.write(&packet) {
                        eprintln!("[-] Falha ao transmitir pacote USB HID: {}. Reconectando...", e);
                        hid_device = None;
                    }
                }
            } else {
                api_opt = hidapi::HidApi::new().ok();
            }

            let sleep_secs = TftSpeed::from_index(speed_idx).interval_secs();
            thread::sleep(Duration::from_secs(sleep_secs.max(1)));
        }
    });

    // Configuração da Janela Nativa
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("ALSEYE BEM")
            .with_inner_size([960.0, 540.0])
            .with_min_inner_size([900.0, 500.0])
            .with_decorations(true),
        ..Default::default()
    };

    eframe::run_native(
        "ALSEYE BEM",
        options,
        Box::new(move |cc| {
            Ok(Box::new(AlseyeApp::new(
                cc,
                config,
                config_path,
                metrics_arc,
                sys_info,
            )))
        }),
    )
}

