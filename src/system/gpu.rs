use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

const REFRESH_INTERVAL: Duration = Duration::from_millis(1500);

pub struct GpuInfo {
    pub available: bool,
    pub gpu_usage: u32,
    pub vram_used: u64,
    pub vram_total: u64,
    pub gpu_freq_mhz: Option<u32>,
    pub mem_freq_mhz: Option<u32>,
    pub power_w: f64,
    pub power_cap_w: Option<f64>,
    pub temp_celsius: f32,
    pub temp_max: f32,
    pub manufacturer: String,
    pub pci_slot: String,
    pub driver: String,
    pub pcie_link: String,
    last_refresh: Instant,
}

fn read_file(path: &PathBuf) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

fn read_at(base: &PathBuf, name: &str) -> Option<String> {
    read_file(&base.join(name))
}

fn read_u64(base: &PathBuf, name: &str) -> Option<u64> {
    read_at(base, name)?.parse().ok()
}

fn find_gpu_device() -> Option<PathBuf> {
    let dir = PathBuf::from("/sys/class/drm");
    let mut entries: Vec<_> = fs::read_dir(&dir).ok()?.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("card") {
            let suffix = &name[4..];
            if !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit()) {
                let device = entry.path().join("device");
                if device.exists() {
                    return Some(device);
                }
            }
        }
    }
    None
}

fn find_hwmon(device: &PathBuf) -> Option<PathBuf> {
    let hwmon_dir = device.join("hwmon");
    fs::read_dir(&hwmon_dir)
        .ok()?
        .filter_map(|e| e.ok())
        .next()
        .map(|e| e.path())
}

fn parse_active_clock_mhz(content: &str) -> Option<u32> {
    // Lines like: "0: 500Mhz\n1: 800Mhz *\n"
    for line in content.lines() {
        if line.contains('*') {
            let lower = line.to_lowercase();
            if let Some(pos) = lower.find("mhz") {
                let num_str = lower[..pos].trim().split_whitespace().last()?;
                return num_str.parse().ok();
            }
        }
    }
    None
}

fn pcie_gen_from_speed(speed: &str) -> &'static str {
    if speed.contains("2.5") { "1.0" }
    else if speed.contains("5.0") { "2.0" }
    else if speed.contains("8.0") { "3.0" }
    else if speed.contains("16.0") { "4.0" }
    else if speed.contains("32.0") { "5.0" }
    else { "?" }
}

fn vendor_to_name(vendor: &str) -> String {
    match vendor.trim().to_lowercase().as_str() {
        "0x1002" => "Advanced Micro Devices, Inc. [AMD/ATI]".to_string(),
        "0x10de" => "NVIDIA Corporation".to_string(),
        "0x8086" => "Intel Corporation".to_string(),
        other => other.to_string(),
    }
}

fn parse_uevent(device: &PathBuf) -> (String, String) {
    let mut driver = "Unknown".to_string();
    let mut pci_slot = "N/A".to_string();
    if let Some(content) = read_at(device, "uevent") {
        for line in content.lines() {
            if let Some(v) = line.strip_prefix("DRIVER=") {
                driver = v.to_string();
            } else if let Some(v) = line.strip_prefix("PCI_SLOT_NAME=") {
                pci_slot = v.to_string();
            }
        }
    }
    (driver, pci_slot)
}

impl GpuInfo {
    pub fn new() -> Self {
        let mut info = Self {
            available: false,
            gpu_usage: 0,
            vram_used: 0,
            vram_total: 0,
            gpu_freq_mhz: None,
            mem_freq_mhz: None,
            power_w: 0.0,
            power_cap_w: None,
            temp_celsius: 0.0,
            temp_max: 0.0,
            manufacturer: "Unknown".to_string(),
            pci_slot: "N/A".to_string(),
            driver: "Unknown".to_string(),
            pcie_link: "N/A".to_string(),
            last_refresh: Instant::now() - REFRESH_INTERVAL,
        };
        info.refresh();
        info
    }

    pub fn refresh_if_needed(&mut self) {
        if self.last_refresh.elapsed() >= REFRESH_INTERVAL {
            self.refresh();
        }
    }

    fn refresh(&mut self) {
        self.last_refresh = Instant::now();

        let Some(device) = find_gpu_device() else {
            self.available = false;
            return;
        };
        self.available = true;

        self.gpu_usage = read_u64(&device, "gpu_busy_percent").unwrap_or(0) as u32;
        self.vram_used = read_u64(&device, "mem_info_vram_used").unwrap_or(0);
        self.vram_total = read_u64(&device, "mem_info_vram_total").unwrap_or(0);

        self.gpu_freq_mhz = read_at(&device, "pp_dpm_sclk")
            .as_deref()
            .and_then(parse_active_clock_mhz);
        self.mem_freq_mhz = read_at(&device, "pp_dpm_mclk")
            .as_deref()
            .and_then(parse_active_clock_mhz);

        if let Some(hwmon) = find_hwmon(&device) {
            let temp = read_u64(&hwmon, "temp1_input").unwrap_or(0) as f32 / 1000.0;
            self.temp_celsius = temp;
            if temp > self.temp_max {
                self.temp_max = temp;
            }
            self.power_w = read_u64(&hwmon, "power1_average")
                .unwrap_or(0) as f64 / 1_000_000.0;
            self.power_cap_w = read_u64(&hwmon, "power1_cap")
                .map(|v| v as f64 / 1_000_000.0);
        }

        let speed = read_at(&device, "current_link_speed").unwrap_or_default();
        let width = read_at(&device, "current_link_width").unwrap_or_default();
        if !speed.is_empty() && !width.is_empty() {
            self.pcie_link = format!("PCIe {} \u{00d7}{}", pcie_gen_from_speed(&speed), width);
        } else {
            self.pcie_link = "N/A".to_string();
        }

        self.manufacturer = read_at(&device, "vendor")
            .map(|v| vendor_to_name(&v))
            .unwrap_or_else(|| "Unknown".to_string());

        let (driver, pci_slot) = parse_uevent(&device);
        self.driver = driver;
        self.pci_slot = pci_slot;
    }
}
