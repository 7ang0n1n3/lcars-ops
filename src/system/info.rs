use std::collections::HashSet;
use std::fs;
use std::time::{Duration, Instant};

use sysinfo::{Components, Disks, Networks, ProcessesToUpdate, System, Users};

const REFRESH_INTERVAL: Duration = Duration::from_millis(1500);

pub struct SystemInfo {
    pub system: System,
    pub networks: Networks,
    pub disks: Disks,
    pub components: Components,
    pub users: Users,
    // CPU sensor
    pub cpu_temp: f32,
    pub cpu_temp_max: f32,
    // CPU static properties (read once at init)
    pub cpu_max_freq_ghz: f32,
    pub cpu_logical_cores: usize,
    pub cpu_physical_cores: usize,
    pub cpu_sockets: usize,
    pub cpu_virtualization: String,
    pub cpu_architecture: String,
    last_refresh: Instant,
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut system = System::new_all();
        // Dual refresh for accurate initial CPU readings
        std::thread::sleep(Duration::from_millis(200));
        system.refresh_all();

        let networks = Networks::new_with_refreshed_list();
        let disks = Disks::new_with_refreshed_list();
        let components = Components::new_with_refreshed_list();
        let users = Users::new_with_refreshed_list();

        let cpu_temp = find_cpu_temp(&components);
        let cpu_logical_cores = system.cpus().len();
        let cpu_physical_cores = system.physical_core_count().unwrap_or(0);

        Self {
            cpu_temp,
            cpu_temp_max: cpu_temp,
            cpu_max_freq_ghz: read_max_freq_ghz(),
            cpu_logical_cores,
            cpu_physical_cores,
            cpu_sockets: count_sockets(),
            cpu_virtualization: detect_virtualization(),
            cpu_architecture: std::env::consts::ARCH.to_string(),
            system,
            networks,
            disks,
            components,
            users,
            last_refresh: Instant::now(),
        }
    }

    pub fn refresh_if_needed(&mut self) -> bool {
        if self.last_refresh.elapsed() >= REFRESH_INTERVAL {
            self.system.refresh_memory();
            self.system.refresh_cpu_all();
            self.system.refresh_processes(ProcessesToUpdate::All, true);
            self.networks.refresh(true);
            self.disks.refresh(true);
            self.components.refresh(true);
            let temp = find_cpu_temp(&self.components);
            self.cpu_temp = temp;
            if temp > self.cpu_temp_max {
                self.cpu_temp_max = temp;
            }
            self.last_refresh = Instant::now();
            true
        } else {
            false
        }
    }

    pub fn cpu_total(&self) -> f32 {
        self.system.global_cpu_usage()
    }

    pub fn cpu_per_core(&self) -> Vec<f32> {
        self.system.cpus().iter().map(|c| c.cpu_usage()).collect()
    }

    pub fn memory_used(&self) -> u64 {
        self.system.used_memory()
    }

    pub fn memory_total(&self) -> u64 {
        self.system.total_memory()
    }

    pub fn memory_fraction(&self) -> f32 {
        let total = self.system.total_memory() as f64;
        if total == 0.0 {
            return 0.0;
        }
        (self.system.used_memory() as f64 / total) as f32
    }

    pub fn swap_fraction(&self) -> f32 {
        let total = self.system.total_swap() as f64;
        if total == 0.0 {
            return 0.0;
        }
        (self.system.used_swap() as f64 / total) as f32
    }

    pub fn swap_used(&self) -> u64 {
        self.system.used_swap()
    }

    pub fn swap_total(&self) -> u64 {
        self.system.total_swap()
    }

    pub fn disk_info(&self) -> Vec<DiskData> {
        self.disks
            .iter()
            .filter(|d| d.total_space() > 0)
            .map(|d| {
                let total = d.total_space();
                let available = d.available_space();
                let used = total.saturating_sub(available);
                DiskData {
                    mount: d.mount_point().to_string_lossy().to_string(),
                    used,
                    total,
                    fraction: used as f32 / total as f32,
                }
            })
            .collect()
    }

    pub fn network_info(&self) -> Vec<NetworkData> {
        let interval = REFRESH_INTERVAL.as_secs_f64();
        self.networks
            .iter()
            .map(|(name, data)| {
                let rx_rate = data.received() as f64 / interval;
                let tx_rate = data.transmitted() as f64 / interval;
                NetworkData {
                    name: name.clone(),
                    rx_bytes: data.total_received(),
                    tx_bytes: data.total_transmitted(),
                    rx_rate,
                    tx_rate,
                }
            })
            .collect()
    }
}

// ── CPU helpers ──────────────────────────────────────────────────────────────

fn find_cpu_temp(components: &Components) -> f32 {
    let patterns = ["Tctl", "Tdie", "Package id 0", "CPU Temperature", "CPU"];
    for pat in &patterns {
        if let Some(c) = components.iter().find(|c| c.label().contains(pat)) {
            if let Some(t) = c.temperature() {
                return t;
            }
        }
    }
    components
        .iter()
        .find_map(|c| c.temperature())
        .unwrap_or(0.0)
}

fn read_max_freq_ghz() -> f32 {
    fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
        .ok()
        .and_then(|s| s.trim().parse::<u64>().ok())
        .map(|khz| khz as f32 / 1_000_000.0)
        .unwrap_or(0.0)
}

fn count_sockets() -> usize {
    let mut ids = HashSet::new();
    if let Ok(entries) = fs::read_dir("/sys/devices/system/cpu") {
        for entry in entries.filter_map(|e| e.ok()) {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("cpu") && name[3..].chars().all(|c| c.is_ascii_digit()) {
                if let Ok(id) = fs::read_to_string(entry.path().join("topology/physical_package_id")) {
                    ids.insert(id.trim().to_string());
                }
            }
        }
    }
    ids.len().max(1)
}

fn detect_virtualization() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        if let Some(flags) = content.lines().find(|l| l.starts_with("flags")) {
            if flags.contains("svm") {
                return "AMD-V".to_string();
            } else if flags.contains("vmx") {
                return "Intel VT-x".to_string();
            }
        }
    }
    "None".to_string()
}

// ── Shared utilities ─────────────────────────────────────────────────────────

pub struct DiskData {
    pub mount: String,
    pub used: u64,
    pub total: u64,
    pub fraction: f32,
}

pub struct NetworkData {
    pub name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_rate: f64,
    pub tx_rate: f64,
}

/// Normalize a byte rate to 0.0–1.0 against 125 MB/s (1 Gbps) reference, clamped.
pub fn rate_fraction(rate: f64) -> f32 {
    const MAX_RATE: f64 = 125_000_000.0;
    (rate / MAX_RATE).clamp(0.0, 1.0) as f32
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1_000;
    const MB: u64 = KB * 1_000;
    const GB: u64 = MB * 1_000;
    const TB: u64 = GB * 1_000;

    if bytes >= TB {
        format!("{:.1} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn format_uptime(seconds: u64) -> String {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}
