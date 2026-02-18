# LCARS-OPS

A Star Trek LCARS-themed system monitor built with [egui](https://github.com/emilk/egui).

![LCARS-OPS Screenshot](lcars-ops-screenshot.png)

## Features

- Real-time CPU usage (total + per-core), temperature, and properties (max frequency, core counts, sockets, uptime, virtualization, architecture)
- Memory and swap monitoring
- Disk usage per mount point
- Network RX/TX rates and totals
- Sortable process table (by name, PID, user, memory, CPU%) with columns for process, PID, user, memory, processor, and GPU — expandable child processes
- Battery monitoring — charge level, power draw, health, design capacity, charge cycles, and hardware info
- GPU monitoring — utilization, VRAM usage, clock frequencies, power draw, temperature, and hardware properties
- Stardate display
- Press `Q` to quit

## Platform

Developed and tested on **Arch Linux** with **Hyprland** (Wayland). Other Linux distributions should work as long as the standard sysfs paths are available (`/sys/class/drm/`, `/sys/class/power_supply/`, etc.). Other desktop environments and window managers should work too.

Windows and macOS are not supported — GPU and battery data is read directly from Linux sysfs.

## Requirements

- Rust toolchain (stable) — install via [rustup](https://rustup.rs)
- A working GPU/display (egui uses wgpu/OpenGL)

## Download

```bash
git clone https://github.com/7ang0n1n3/lcars-ops.git
cd lcars-ops
```

## Build & Run

```bash
cargo build --release
cargo run --release
```

The compiled binary will be at `target/release/lcars-ops`.
