use std::sync::{Mutex, MutexGuard, OnceLock};
use sysinfo::{ProcessesToUpdate, System};

static SYSTEM: OnceLock<Mutex<System>> = OnceLock::new();

pub fn get_system() -> MutexGuard<'static, System> {
    let system = SYSTEM.get_or_init(|| Mutex::new(System::new_all()));

    system.lock().expect("System mutex was poisoned")
}

pub fn refresh_system() -> MutexGuard<'static, System> {
    let mut system = get_system();
    system.refresh_all();
    system
}

pub fn refresh_processes() -> MutexGuard<'static, System> {
    let mut system = get_system();
    system.refresh_processes(ProcessesToUpdate::All, false);
    system
}

pub fn refresh_memory() -> MutexGuard<'static, System> {
    let mut system = get_system();
    system.refresh_memory();
    system
}

pub fn refresh_cpu() -> MutexGuard<'static, System> {
    let mut system = get_system();
    system.refresh_cpu_all();
    system
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_available_memory() {
        let system = super::refresh_system();
        let available_memory = system.available_memory();

        println!("Available memory: {} bytes", available_memory);
        println!("Available memory: {:.2} MB", bytes_to_mb(available_memory));
        println!("Available memory: {:.2} GB", bytes_to_gb(available_memory));
    }

    #[test]
    fn test_boot_time() {
        let boot_time = sysinfo::System::boot_time();

        println!("Boot time: {} seconds", boot_time);
        println!("Boot time: {:.2} minutes", boot_time as f64 / 60.0);
        println!("Boot time: {:.2} hours", boot_time as f64 / 3600.0);
        println!("Boot time: {:.2} days", boot_time as f64 / 86400.0);
        println!("Boot time: {:.2} years", boot_time as f64 / 31536000.0);
    }

    #[test]
    fn test_cgroup_limits() {
        let system = super::refresh_system();
        let cgroup_limits = system.cgroup_limits();

        println!("Cgroup limits: {:?}", cgroup_limits);
    }

    #[test]
    fn test_cpu_arch() {
        let cpu_arch = sysinfo::System::cpu_arch();

        println!("CPU architecture: {:?}", cpu_arch);
    }

    #[test]
    fn test_cpus() {
        let system = super::refresh_system();
        let cpus = system.cpus();

        assert!(!cpus.is_empty(), "CPUs should not be empty");
    }

    #[test]
    fn test_distribution_id() {
        let distribution_id = sysinfo::System::distribution_id();

        println!("Distribution ID: {:?}", distribution_id);
    }

    #[test]
    fn test_distribution_id_like() {
        let distribution_id_like = sysinfo::System::distribution_id_like();

        println!("Distribution ID like: {:?}", distribution_id_like);
    }

    #[test]
    fn test_free_memory() {
        let system = super::refresh_system();
        let free_memory = system.free_memory();

        println!("Free memory: {} bytes", free_memory);
        println!("Free memory: {:.2} MB", bytes_to_mb(free_memory));
        println!("Free memory: {:.2} GB", bytes_to_gb(free_memory));
    }

    #[test]
    fn test_free_swap() {
        let system = super::refresh_system();
        let free_swap = system.free_swap();

        println!("Free swap: {} bytes", free_swap);
        println!("Free swap: {:.2} MB", bytes_to_mb(free_swap));
        println!("Free swap: {:.2} GB", bytes_to_gb(free_swap));
    }

    #[test]
    fn test_global_cpu_usage() {
        let system = super::refresh_system();
        let global_cpu_usage = system.global_cpu_usage();

        println!("Global CPU usage: {:.2}%", global_cpu_usage * 1.0);
    }

    #[test]
    fn test_hostname() {
        let hostname = sysinfo::System::host_name().unwrap();

        println!("Hostname: {:?}", hostname);
    }

    #[test]
    fn test_kernel_version() {
        let kernel_version = sysinfo::System::kernel_version();

        println!("Kernel version: {:?}", kernel_version);
    }

    #[test]
    fn test_kernel_long_version() {
        let kernel_long_version = sysinfo::System::kernel_long_version();

        println!("Kernel long version: {:?}", kernel_long_version);
    }

    #[test]
    fn test_load_average() {
        let load_average = sysinfo::System::load_average();

        println!("Load average: {:?}", load_average);
    }

    #[test]
    fn test_long_os_name() {
        let long_os_name = sysinfo::System::long_os_version().unwrap();

        println!("Long OS name: {:?}", long_os_name);
    }

    #[test]
    fn test_name() {
        let name = sysinfo::System::name().unwrap();

        println!("Name: {:?}", name);
    }

    #[test]
    fn test_os_version() {
        let os_version = sysinfo::System::os_version().unwrap();

        println!("OS version: {:?}", os_version);
    }

    #[test]
    fn test_physical_core_count() {
        let physical_core_count = sysinfo::System::physical_core_count().unwrap();

        println!("Physical core count: {:?}", physical_core_count);
    }

    #[test]
    fn test_processes() {
        let system = super::refresh_system();
        let processes = system.processes();

        assert!(!processes.is_empty(), "Processes should not be empty");
    }

    #[test]
    fn test_process() {
        let system = super::refresh_system();
        let processes = system.processes();
        let mut success_count = 0;
        for process in processes.values() {
            let pid = process.pid();
            let process_2 = system.process(pid);
            assert_eq!(process_2.unwrap().pid(), pid, "Process ID should match");
            success_count += 1;
        }
        println!("Successfully checked {} processes", success_count);
    }

    #[test]
    fn test_total_memory() {
        let system = super::refresh_system();
        let total_memory = system.total_memory();

        println!("Total memory: {} bytes", total_memory);
        println!("Total memory: {:.2} MB", bytes_to_mb(total_memory));
        println!("Total memory: {:.2} GB", bytes_to_gb(total_memory));
    }

    #[test]
    fn test_total_swap() {
        let system = super::refresh_system();
        let total_swap = system.total_swap();

        println!("Total swap: {} bytes", total_swap);
        println!("Total swap: {:.2} MB", bytes_to_mb(total_swap));
        println!("Total swap: {:.2} GB", bytes_to_gb(total_swap));
    }

    #[test]
    fn test_up_time() {
        let up_time = sysinfo::System::uptime();

        println!("Up time: {} seconds", up_time);
        println!("Up time: {:.2} minutes", up_time as f64 / 60.0);
        println!("Up time: {:.2} hours", up_time as f64 / 3600.0);
        println!("Up time: {:.2} days", up_time as f64 / 86400.0);
        println!("Up time: {:.2} years", up_time as f64 / 31536000.0);
    }

    #[test]
    fn test_used_memory() {
        let system = super::refresh_system();
        let used_memory = system.used_memory();

        println!("Used memory: {} bytes", used_memory);
        println!("Used memory: {:.2} MB", bytes_to_mb(used_memory));
        println!("Used memory: {:.2} GB", bytes_to_gb(used_memory));
    }

    #[test]
    fn test_used_swap() {
        let system = super::refresh_system();
        let used_swap = system.used_swap();

        println!("Used swap: {} bytes", used_swap);
        println!("Used swap: {:.2} MB", bytes_to_mb(used_swap));
        println!("Used swap: {:.2} GB", bytes_to_gb(used_swap));
    }

    #[test]
    fn test_system_singleton() {
        // Initialize the OnceLock first
        let _system = super::get_system();

        // Get the address of the Mutex itself, not the MutexGuard
        let system1_ptr = super::SYSTEM.get().unwrap() as *const _ as usize;
        println!("System pointer: {:?}", system1_ptr);
        let system2_ptr = super::SYSTEM.get().unwrap() as *const _ as usize;
        println!("System pointer: {:?}", system2_ptr);
        assert_eq!(
            system1_ptr, system2_ptr,
            "System instances should be the same"
        );
    }

    #[test]
    fn test_refresh_processes() {
        let system = super::refresh_processes();
        let processes = system.processes();

        assert!(!processes.is_empty(), "Processes should not be empty");
    }

    #[test]
    fn test_refresh_memory() {
        let system = super::refresh_memory();
        let total_memory = system.total_memory();

        println!("Total memory: {} bytes", total_memory);
        println!("Total memory: {:.2} MB", bytes_to_mb(total_memory));
        println!("Total memory: {:.2} GB", bytes_to_gb(total_memory));
    }

    #[test]
    fn test_refresh_cpu() {
        {
            let system = super::get_system();
            assert!(!system.cpus().is_empty(), "CPUs should not be empty");
        }
        let system = super::refresh_cpu();
        let cpu_usage = system.global_cpu_usage();

        println!("Global CPU usage: {:.2}%", cpu_usage * 1.0);
    }

    fn bytes_to_mb(bytes: u64) -> f64 {
        (bytes as f64) / 1024.0 / 1024.0
    }

    fn bytes_to_gb(bytes: u64) -> f64 {
        bytes_to_mb(bytes) / 1024.0
    }

    #[test]
    fn test_cpu_brand() {
        let system = super::refresh_cpu();
        let cpu_brand = system.cpus()[0].brand();

        println!("CPU brand: {:?}", cpu_brand);
    }

    #[test]
    fn test_cpu_usage() {
        {
            let system = super::refresh_cpu();
            assert!(!system.cpus().is_empty(), "CPUs should not be empty");
        }
        // wait a bit();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let system = super::refresh_cpu();
        let cpu_usage = system.cpus()[0].cpu_usage();

        println!("CPU usage: {:.2}%", cpu_usage);
    }

    #[test]
    fn test_cpu_frequency() {
        {
            let system = super::refresh_cpu();
            assert!(!system.cpus().is_empty(), "CPUs should not be empty");
        }
        let system = super::refresh_cpu();
        let cpu_frequency = system.cpus()[0].frequency();

        println!("CPU frequency: {} MHz", cpu_frequency);
    }
}
