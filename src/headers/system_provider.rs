pub mod system_provider {
    use std::sync::{Arc, Mutex};
    use crate::headers::config;

    pub struct SysInfo {
        pub cpuUsage: Vec<f32>,
    }

    pub fn start(sys: Arc<Mutex<sysinfo::System>>) {
        std::thread::spawn(move || {
            loop {
                sys.lock().unwrap().refresh_all();

                // Wait a bit because CPU usage is based on diff.
                std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
                // Refresh CPUs again to get actual value.
                sys.lock().unwrap().refresh_cpu_all();

                std::thread::sleep(std::time::Duration::from_millis(config::REFRESH_INTERVAL));
            }
        });
    }

    pub fn getSysInfo(sys: &sysinfo::System) -> SysInfo {
        let mut cpuUsage: Vec<f32> = Vec::new();

        for (i, cpu) in sys.cpus().iter().enumerate() {
            cpuUsage.push(cpu.cpu_usage());
        }
        return SysInfo {
            cpuUsage
        }
    }
}
