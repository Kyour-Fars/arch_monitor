use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use sysinfo::{CpuExt, NetworkExt, NetworksExt, System, SystemExt};
use std::collections::VecDeque;

const HISTORY_LENGTH: usize = 100;

pub struct ArchMonitorApp {
    sys: System,

    cpu_history: VecDeque<f32>,
    mem_history: VecDeque<f32>,
    net_recv_history: VecDeque<f64>,
    net_transmit_history: VecDeque<f64>,

    prev_net_received: u64,
    prev_net_transmitted: u64,
}

impl Default for ArchMonitorApp {
    fn default() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self {
            sys,
            cpu_history: VecDeque::with_capacity(HISTORY_LENGTH),
            mem_history: VecDeque::with_capacity(HISTORY_LENGTH),
            net_recv_history: VecDeque::with_capacity(HISTORY_LENGTH),
            net_transmit_history: VecDeque::with_capacity(HISTORY_LENGTH),
            prev_net_received: 0,
            prev_net_transmitted: 0,
        }
    }
}

impl eframe::App for ArchMonitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.sys.refresh_all();

        // CPU usage (%)
        let cpu_usage = self.sys.global_cpu_info().cpu_usage();
        self.cpu_history.push_back(cpu_usage);
        if self.cpu_history.len() > HISTORY_LENGTH {
            self.cpu_history.pop_front();
        }

        // Memory usage (MiB)
        let used_memory_mib = (self.sys.used_memory() / 1024) as f32;
        self.mem_history.push_back(used_memory_mib);
        if self.mem_history.len() > HISTORY_LENGTH {
            self.mem_history.pop_front();
        }

        // Network bytes (sum across all interfaces)
        let total_received: u64 = self.sys.networks().iter().map(|(_, data)| data.received()).sum();
        let total_transmitted: u64 = self.sys.networks().iter().map(|(_, data)| data.transmitted()).sum();

        let delta_received = if total_received >= self.prev_net_received {
            (total_received - self.prev_net_received) as f64
        } else {
            0.0
        };
        let delta_transmitted = if total_transmitted >= self.prev_net_transmitted {
            (total_transmitted - self.prev_net_transmitted) as f64
        } else {
            0.0
        };

        self.prev_net_received = total_received;
        self.prev_net_transmitted = total_transmitted;

        self.net_recv_history.push_back(delta_received);
        self.net_transmit_history.push_back(delta_transmitted);

        if self.net_recv_history.len() > HISTORY_LENGTH {
            self.net_recv_history.pop_front();
        }
        if self.net_transmit_history.len() > HISTORY_LENGTH {
            self.net_transmit_history.pop_front();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Arch Monitor");

            // Display current values
            ui.horizontal(|ui| {
                ui.label(format!("CPU Usage: {:.2}%", cpu_usage));
                ui.label(format!(
                    "Memory Used: {} MiB / {} MiB",
                    self.sys.used_memory() / 1024,
                    self.sys.total_memory() / 1024
                ));
            });

            ui.separator();

            // CPU Usage Graph
            ui.label("CPU Usage (%)");
            let cpu_points: PlotPoints = self
                .cpu_history
                .iter()
                .enumerate()
                .map(|(i, &v)| [i as f64, v as f64])
                .collect();
            Plot::new("cpu_plot")
                .view_aspect(4.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(Line::new(cpu_points));
                });

            ui.separator();

            // Memory Usage Graph
            ui.label("Memory Usage (MiB)");
            let mem_points: PlotPoints = self
                .mem_history
                .iter()
                .enumerate()
                .map(|(i, &v)| [i as f64, v as f64])
                .collect();
            Plot::new("mem_plot")
                .view_aspect(4.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(Line::new(mem_points));
                });

            ui.separator();

            // Network Usage Graph (Received + Transmitted)
            ui.label("Network Traffic (Bytes per update)");
            let net_recv_points: PlotPoints = self
                .net_recv_history
                .iter()
                .enumerate()
                .map(|(i, &v)| [i as f64, v])
                .collect();
            let net_transmit_points: PlotPoints = self
                .net_transmit_history
                .iter()
                .enumerate()
                .map(|(i, &v)| [i as f64, v])
                .collect();

            Plot::new("net_plot")
                .view_aspect(4.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(Line::new(net_recv_points).name("Received").color(egui::Color32::GREEN));
                    plot_ui.line(Line::new(net_transmit_points).name("Transmitted").color(egui::Color32::RED));
                    // No legend method, so skip
                });

            ui.separator();

            // Show current total network stats per interface
            for (iface, data) in self.sys.networks() {
                ui.label(format!(
                    "{}: RX {} B, TX {} B",
                    iface,
                    data.received(),
                    data.transmitted()
                ));
            }
        });

        // Request repaint every frame for smooth animation
        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Arch Monitor",
        options,
        Box::new(|_cc| Box::new(ArchMonitorApp::default())),
    )
}
