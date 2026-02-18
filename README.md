# LCARS-OPS

A Star Trek LCARS-themed system monitor built with [egui](https://github.com/emilk/egui).

![LCARS-OPS Screenshot](lcars-ops-screenshot.png)

## Features

- Real-time CPU usage (total + per-core)
- Memory and swap monitoring
- Disk usage per mount point
- Network RX/TX rates and totals
- Sortable process table with expandable child processes
- Battery monitoring — charge level, power draw, health, design capacity, charge cycles, and hardware info
- Stardate display
- Press `Q` to quit

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
