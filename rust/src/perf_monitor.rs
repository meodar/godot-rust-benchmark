use godot::classes::{Engine, ITimer, Timer};
use godot::prelude::*;
use nvml_wrapper::Nvml;
use std::io::Write;
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};
use sysinfo::{Pid, ProcessesToUpdate, System};

#[derive(GodotClass)]
#[class(base=Timer)]
pub struct PerfMonitor {
    pid: Option<Pid>,
    system: Option<System>,
    nvml: Option<Nvml>,
    csv_file: Option<File>,
    delta_time: f64,

    #[base]
    base: Base<Timer>,
}

impl PerfMonitor {
    fn on_timeout(perf_monitor: &mut PerfMonitor) {
        let mut cpu = 0.0;
        let mut ram = 0.0;
        let mut gpu = 0;
        let mut gpu_mem = 0;
        let fps = Engine::singleton().get_frames_per_second();

        if let Some(system) = &mut perf_monitor.system {
            system.refresh_processes(ProcessesToUpdate::All, true);
            if let Some(pid) = perf_monitor.pid {
                if let Some(process) = system.process(pid) {
                    cpu = process.cpu_usage();
                    ram = process.memory() as f32 / 1024.0 / 1024.0;
                    println!("CPU usage: {:.2}%", cpu);
                    println!("RAM usage: {:.2} MiB", ram);
                }
                if let Some(nvml) = &perf_monitor.nvml {
                    if let Ok(device0) = nvml.device_by_index(0) {
                        if let Ok(stats) = device0.process_utilization_stats(None) {
                            if let Some(util) = stats.iter().find(|s| s.pid == pid.as_u32()) {
                                gpu = util.sm_util;
                                gpu_mem = util.mem_util;
                                println!(
                                    "GPU {} usage (this process): {}% | Memory: {} MiB",
                                    device0.name().unwrap_or_default(),
                                    gpu,
                                    gpu_mem
                                );
                            }
                        }
                    }
                }
            }
        }
        println!("FPS: {}", fps);

        if let Some(file) = &mut perf_monitor.csv_file {
            writeln!(
                file,
                "{},{:.2},{:.2},{},{},{}",
                perf_monitor.delta_time, cpu, ram, gpu, gpu_mem, fps
            )
            .expect("Failed to write to CSV");
        }
    }
}

#[godot_api]
impl ITimer for PerfMonitor {
    fn init(base: Base<Timer>) -> Self {
        PerfMonitor {
            base,
            pid: None,
            system: None,
            nvml: None,
            csv_file: None,
            delta_time: 0.0,
        }
    }

    fn ready(&mut self) {
        let now = chrono::Local::now();
        let filename = format!("perf_{}.csv", now.format("%Y-%m-%d_%H-%M-%S"));
        let mut csv_path = PathBuf::from("./");
        csv_path.push(filename);

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&csv_path)
            .expect("Failed to create CSV file");

        writeln!(file, "second,cpu_percent,ram_mib,gpu_percent,gpu_mem,fps").unwrap();

        self.csv_file = Some(file);

        let nvml = Some(Nvml::init().expect("failed to initialize NVML"));
        let mut system: System = System::new_all();
        system.refresh_all();

        println!("PID: {}", std::process::id());

        self.pid = Some(Pid::from_u32(std::process::id()));
        self.system = Some(system);
        self.nvml = nvml;

        self.base()
            .signals()
            .timeout()
            .connect_other(self, Self::on_timeout);
    }

    fn physics_process(&mut self, delta: f64) {
        self.delta_time += delta
    }
}
