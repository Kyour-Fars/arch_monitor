# 🖥️ arch_monitor

[![GitHub](https://img.shields.io/github/license/Kyour-Fars/arch_monitor?color=4caf50)](https://github.com/Kyour-Fars/arch_monitor/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/built%20with-rust-orange?logo=rust)](https://www.rust-lang.org/)
[![UI: egui](https://img.shields.io/badge/ui-egui-8e44ad)](https://github.com/emilk/egui)

_______________________________________________

A minimal yet visually polished system monitor for **Arch Linux**, built with Rust and egui.  
Displays real-time CPU, memory, and network usage using interactive charts and a modern GUI inspired by Windows Task Manager.

_______________________________________________

## 📊 Features

- Real-time telemetry:
  - CPU usage
  - Memory usage
  - Network activity
- Graphs with smooth, dynamic updates
- Custom dark-themed interface

_______________________________________________

## 🔧 Building from Source

```bash
git clone https://github.com/Kyour-Fars/arch_monitor.git
cd arch_monitor
cargo run --release

| Component         | Role                        |
| ----------------- | --------------------------- |
| `Rust`            | Performance & memory safety |
| `eframe` / `egui` | Native GUI toolkit          |
| `sysinfo`         | System statistics backend   |
