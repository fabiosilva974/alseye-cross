//! Protocolo USB HID do ALSEYE BEM
//! Descompilado do ALSEYE BEM.exe (rotinas 0x40a340 e 0x40bd30)

pub const VENDOR_ID: u16 = 0x5131;
pub const PRODUCT_ID: u16 = 0x2007;

/// Modos de rotação da tela TFT
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Rotation {
    Deg0 = 0x02,
    Deg90 = 0x12,
    Deg180 = 0x22,
    Deg270 = 0x32,
}

impl Rotation {
    pub fn from_index(index: u8) -> Self {
        match index {
            1 => Rotation::Deg90,
            2 => Rotation::Deg180,
            3 => Rotation::Deg270,
            _ => Rotation::Deg0,
        }
    }

    pub fn to_degrees(self) -> u16 {
        match self {
            Rotation::Deg0 => 0,
            Rotation::Deg90 => 90,
            Rotation::Deg180 => 180,
            Rotation::Deg270 => 270,
        }
    }
}

/// Intervalo do TFT_SPEED
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TftSpeed {
    Slow = 0,   // 5 segundos
    Normal = 1, // 3 segundos
    Fast = 2,   // 1 segundo
}

impl TftSpeed {
    pub fn from_index(idx: u8) -> Self {
        match idx {
            0 => TftSpeed::Slow,
            1 => TftSpeed::Normal,
            _ => TftSpeed::Fast,
        }
    }

    pub fn interval_secs(self) -> u64 {
        match self {
            TftSpeed::Slow => 5,
            TftSpeed::Normal => 3,
            TftSpeed::Fast => 1,
        }
    }
}

/// Monta o pacote Standard (65 bytes)
/// Byte 0: 0x00 (Report ID)
/// Byte 1: 0x40 ('@')
/// Byte 2: Temperatura da CPU em Celsius
pub fn build_standard_report(cpu_temp: u8) -> [u8; 65] {
    let mut report = [0u8; 65];
    report[0] = 0x00;
    report[1] = 0x40;
    report[2] = cpu_temp;
    report
}

/// Monta o pacote TFT Display (18 bytes dentro de 65 bytes)
/// Header: 0xC5, Rotação, CPU Temp, GPU Temp, RPM Fan, Checksum
pub fn build_tft_report(rotation: Rotation, cpu_temp: u8, gpu_temp: u8, rpm: u16) -> [u8; 65] {
    let mut payload = [0u8; 18];
    payload[0] = 0xC5; // Header de sincronização
    payload[1] = rotation as u8;
    payload[2] = 0x00;
    payload[3] = cpu_temp;
    payload[4] = gpu_temp;
    payload[5] = ((rpm >> 8) & 0xFF) as u8;
    payload[6] = (rpm & 0xFF) as u8;

    // Checksum: soma dos primeiros 17 bytes (0..16) mod 256
    let sum: u32 = payload[0..17].iter().map(|&b| b as u32).sum();
    payload[17] = (sum & 0xFF) as u8;

    let mut report = [0u8; 65];
    report[0] = 0x00; // Report ID
    report[1..19].copy_from_slice(&payload);
    report
}

