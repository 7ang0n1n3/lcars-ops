use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

const REFRESH_INTERVAL: Duration = Duration::from_secs(5);

pub struct BatteryInfo {
    pub device: String,
    pub capacity: u32,
    pub status: String,
    pub health: f32,
    pub energy_full_design: f64, // Wh
    pub power_now: f64,          // W
    pub cycle_count: u32,
    pub technology: String,
    pub manufacturer: String,
    pub model_name: String,
    pub available: bool,
    last_refresh: Instant,
}

fn read_sysfs(base: &PathBuf, name: &str) -> Option<String> {
    fs::read_to_string(base.join(name))
        .ok()
        .map(|s| s.trim().to_string())
}

fn read_u64(base: &PathBuf, name: &str) -> Option<u64> {
    read_sysfs(base, name)?.parse().ok()
}

fn find_battery() -> Option<PathBuf> {
    let dir = PathBuf::from("/sys/class/power_supply");
    fs::read_dir(&dir).ok()?.find_map(|entry| {
        let path = entry.ok()?.path();
        let type_str = fs::read_to_string(path.join("type")).ok()?;
        if type_str.trim() == "Battery" {
            Some(path)
        } else {
            None
        }
    })
}

impl BatteryInfo {
    pub fn new() -> Self {
        let mut info = Self {
            device: String::new(),
            capacity: 0,
            status: "Unknown".to_string(),
            health: 0.0,
            energy_full_design: 0.0,
            power_now: 0.0,
            cycle_count: 0,
            technology: "Unknown".to_string(),
            manufacturer: "Unknown".to_string(),
            model_name: "Unknown".to_string(),
            available: false,
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

        let Some(path) = find_battery() else {
            self.available = false;
            return;
        };

        self.available = true;
        self.device = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        self.capacity = read_u64(&path, "capacity").unwrap_or(0) as u32;
        self.status = read_sysfs(&path, "status").unwrap_or_else(|| "Unknown".to_string());
        self.technology = read_sysfs(&path, "technology").unwrap_or_else(|| "Unknown".to_string());
        self.manufacturer = read_sysfs(&path, "manufacturer").unwrap_or_else(|| "Unknown".to_string());
        self.model_name = read_sysfs(&path, "model_name").unwrap_or_else(|| "Unknown".to_string());
        self.cycle_count = read_u64(&path, "cycle_count").unwrap_or(0) as u32;

        let voltage_now = read_u64(&path, "voltage_now").unwrap_or(0) as f64; // µV

        // Energy values in µWh; fall back to charge (µAh) * voltage (µV) / 1e12
        let energy_full = read_u64(&path, "energy_full")
            .map(|v| v as f64 / 1_000_000.0)
            .or_else(|| {
                let c = read_u64(&path, "charge_full")? as f64;
                Some(c * voltage_now / 1_000_000_000_000.0)
            })
            .unwrap_or(0.0);

        let energy_full_design = read_u64(&path, "energy_full_design")
            .map(|v| v as f64 / 1_000_000.0)
            .or_else(|| {
                let c = read_u64(&path, "charge_full_design")? as f64;
                Some(c * voltage_now / 1_000_000_000_000.0)
            })
            .unwrap_or(0.0);

        self.energy_full_design = energy_full_design;

        if energy_full_design > 0.0 {
            self.health = (energy_full / energy_full_design * 100.0) as f32;
        }

        // Power in µW; fall back to current (µA) * voltage (µV) / 1e12
        self.power_now = read_u64(&path, "power_now")
            .map(|v| v as f64 / 1_000_000.0)
            .or_else(|| {
                let i = read_u64(&path, "current_now")? as f64;
                Some(i * voltage_now / 1_000_000_000_000.0)
            })
            .unwrap_or(0.0);
    }
}
