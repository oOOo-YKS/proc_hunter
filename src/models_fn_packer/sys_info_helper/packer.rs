use std::sync::{Mutex, MutexGuard, OnceLock};
use sysinfo::{ProcessesToUpdate, System};

// before calling this method, make sure youhave called get_system() at least once
pub fn get_cpu_brief_info() -> (usize, usize, f32, Vec<(String, String, u64, f32)>) {
    let system = refresh_cpu();
    let physical_cpu_core_num = sysinfo::System::physical_core_count().unwrap_or(0);
    let global_cpu_usage = system.global_cpu_usage();
    let cpus = system.cpus();
    let cpu_num = cpus.len();
    if cpu_num == 0 {
        return (physical_cpu_core_num, 0, global_cpu_usage, Vec::new());
    }
    let mut cpus_info = Vec::new();
    for cpu in cpus {
        let cpu_usage = cpu.cpu_usage();
        let cpu_frequency = cpu.frequency();
        let cpu_brand = cpu.brand().to_string();
        let cpu_name = cpu.name().to_string();
        cpus_info.push((cpu_brand, cpu_name, cpu_frequency, cpu_usage));
    }
    return (physical_cpu_core_num, cpu_num, global_cpu_usage, cpus_info);
}

pub fn get_cpu_brief_info_with_1_ms_sleep() -> (usize, usize, f32, Vec<(String, String, u64, f32)>)
{
    let system = refresh_cpu();
    // Sleep for 1 ms to allow CPU usage to be updated
    std::thread::sleep(std::time::Duration::from_millis(1));
    // Refresh the system again to get updated CPU usage
    let physical_cpu_core_num = sysinfo::System::physical_core_count().unwrap_or(0);
    let global_cpu_usage = system.global_cpu_usage();
    let cpus = system.cpus();
    let cpu_num = cpus.len();
    if cpu_num == 0 {
        return (physical_cpu_core_num, 0, global_cpu_usage, Vec::new());
    }
    let mut cpus_info = Vec::new();
    for cpu in cpus {
        let cpu_usage = cpu.cpu_usage();
        let cpu_frequency = cpu.frequency();
        let cpu_brand = cpu.brand().to_string();
        let cpu_name = cpu.name().to_string();
        cpus_info.push((cpu_brand, cpu_name, cpu_frequency, cpu_usage));
    }
    return (physical_cpu_core_num, cpu_num, global_cpu_usage, cpus_info);
}

pub fn get_memory_brief_info() -> (u64, u64, u64, u64, u64, u64) {
    let system = refresh_memory();
    let total_memory = system.total_memory();
    let free_memory = system.free_memory();
    let used_memory = system.used_memory();
    let total_swap = system.total_swap();
    let free_swap = system.free_swap();
    let used_swap = system.used_swap();
    return (
        total_memory,
        free_memory,
        used_memory,
        total_swap,
        free_swap,
        used_swap,
    );
}

pub fn get_processes_brief_info() -> (usize, usize) {
    let system = refresh_processes();
    let processes = system.processes();
    let processes_num = processes.len();
    let mut root = 0;
    for process in processes.values() {
        match process.parent() {
            Some(parent) => {
                if parent == sysinfo::Pid::from_u32(0) {
                    root += 1;
                }
            }
            None => {
                root += 1;
            }
        }
    }
    return (processes_num, root);
}
static SYSTEM: OnceLock<Mutex<System>> = OnceLock::new();

pub fn get_system() -> MutexGuard<'static, System> {
    let system = SYSTEM.get_or_init(|| Mutex::new(System::new_all()));

    system.lock().expect("System mutex was poisoned")
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
        let system = super::get_system();
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
        let system = super::get_system();
        system.cgroup_limits();
    }

    #[test]
    fn test_cpu_arch() {
        let cpu_arch = sysinfo::System::cpu_arch();

        println!("CPU architecture: {:?}", cpu_arch);
    }

    #[test]
    fn test_cpus() {
        let system = super::get_system();
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
        let system = super::get_system();
        let free_memory = system.free_memory();

        println!("Free memory: {} bytes", free_memory);
        println!("Free memory: {:.2} MB", bytes_to_mb(free_memory));
        println!("Free memory: {:.2} GB", bytes_to_gb(free_memory));
    }

    #[test]
    fn test_free_swap() {
        let system = super::get_system();
        let free_swap = system.free_swap();

        println!("Free swap: {} bytes", free_swap);
        println!("Free swap: {:.2} MB", bytes_to_mb(free_swap));
        println!("Free swap: {:.2} GB", bytes_to_gb(free_swap));
    }

    #[test]
    fn test_global_cpu_usage() {
        let system = super::get_system();
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
        let system = super::get_system();
        let processes = system.processes();

        assert!(!processes.is_empty(), "Processes should not be empty");
    }

    #[test]
    fn test_process() {
        let system = super::get_system();
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
        let system = super::get_system();
        let total_memory = system.total_memory();

        println!("Total memory: {} bytes", total_memory);
        println!("Total memory: {:.2} MB", bytes_to_mb(total_memory));
        println!("Total memory: {:.2} GB", bytes_to_gb(total_memory));
    }

    #[test]
    fn test_total_swap() {
        let system = super::get_system();
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
        let system = super::get_system();
        let used_memory = system.used_memory();

        println!("Used memory: {} bytes", used_memory);
        println!("Used memory: {:.2} MB", bytes_to_mb(used_memory));
        println!("Used memory: {:.2} GB", bytes_to_gb(used_memory));
    }

    #[test]
    fn test_used_swap() {
        let system = super::get_system();
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

    #[test]
    fn test_cpu_name() {
        let system = super::refresh_cpu();
        let cpu_name = system.cpus()[0].name();

        println!("CPU name: {:?}", cpu_name);
    }

    #[test]
    fn test_cpu_vendor_id() {
        let system = super::refresh_cpu();
        let cpu_vendor_id = system.cpus()[0].vendor_id();

        println!("CPU vendor ID: {:?}", cpu_vendor_id);
    }

    #[test]
    fn test_process_accumulated_cpu_time() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values() {
            process.accumulated_cpu_time();
        }
    }

    #[test]
    fn test_process_cmd() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let cmd = process.cmd();
            println!("Process command: {:?}", cmd);
        }
    }

    #[test]
    fn test_process_cpu_usage() {
        {
            let system = super::refresh_processes();
            assert!(
                !system.processes().is_empty(),
                "Processes should not be empty"
            );
        }
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let cpu_usage = process.cpu_usage();
            println!("Process CPU usage: {:.2}%", cpu_usage);
        }
    }

    #[test]
    fn test_process_cwd() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let cws = process.cwd();
            println!("Process CWD: {:?}", cws);
        }
    }

    #[test]
    fn test_process_disk_usage() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let disk_usage = process.disk_usage();
            println!("Process disk usage: {:?}", disk_usage);
        }
    }

    #[test]
    fn test_process_effective_group_id() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let effective_group_id = process.effective_group_id();
            println!("Process effective group ID: {:?}", effective_group_id);
        }
    }

    #[test]
    fn test_process_effective_user_id() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let effective_user_id = process.effective_user_id();
            println!("Process effective user ID: {:?}", effective_user_id);
        }
    }

    #[test]
    fn test_process_environ() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let environ = process.environ();
            println!("Process environment: {:?}", environ);
        }
    }
    #[test]
    fn test_process_exe() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let exe = process.exe();
            println!("Process executable: {:?}", exe);
        }
    }

    #[test]
    fn test_process_exists() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let exists = process.exists();
            println!("Process exists: {:?}", exists);
        }
    }

    #[test]
    fn test_process_group_id() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let group_id = process.group_id();
            println!("Process group ID: {:?}", group_id);
        }
    }

    #[test]
    fn test_process_supported_signals() {
        let supported_signals = sysinfo::SUPPORTED_SIGNALS;
        for signal in supported_signals {
            println!("Supported signal: {:?}", signal);
        }
    }

    #[test]
    fn test_process_kill() {
        // Spawn a persistent test process (cross-platform compatible)
        let test_bin = std::env::current_exe().expect("Failed to get test executable path");
        let mut test_process = std::process::Command::new(test_bin)
            .arg("--test-threads=1") // Run an empty test as persistent process
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Failed to spawn test process");

        let pid = sysinfo::Pid::from_u32(test_process.id() as u32);
        println!("Test process PID: {}", pid);

        // Ensure process is registered in the system
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Attempt to kill the process
        let system = super::refresh_processes();
        if let Some(process) = system.process(pid) {
            let result = process.kill();
            println!("Kill result: {:?}", result);
            assert!(result);
        } else {
            let _ = test_process.kill();
            panic!("Process not recognized by sysinfo");
        }
    }

    #[test]
    fn test_process_kill_with() {
        // Spawn a persistent test process (cross-platform compatible)
        let test_bin = std::env::current_exe().expect("Failed to get test executable path");
        let mut test_process = std::process::Command::new(test_bin)
            .arg("--test-threads=1") // Run an empty test as persistent process
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Failed to spawn test process");

        let pid = sysinfo::Pid::from_u32(test_process.id() as u32);
        println!("Test process PID: {}", pid);

        // Ensure process is registered in the system
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Attempt to kill the process
        let system = super::refresh_processes();
        if let Some(process) = system.process(pid) {
            let result = process.kill_with(sysinfo::Signal::Kill).unwrap();
            println!("Kill result: {:?}", result);
            assert!(result);
        } else {
            let _ = test_process.kill();
            panic!("Process not recognized by sysinfo");
        }
    }

    #[test]
    fn test_process_memory() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let memory = process.memory();
            println!("Process memory: {} bytes", memory);
        }
    }

    #[test]
    fn test_process_name() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let name = process.name();
            println!("Process name: {:?}", name);
        }
    }

    #[test]
    fn test_process_open_files() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let open_files = process.open_files();
            println!("Process open files: {:?}", open_files);
        }
    }

    #[test]
    fn test_process_open_files_limit() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let open_files_limit = process.open_files_limit();
            println!("Process open files limit: {:?}", open_files_limit);
        }
    }

    #[test]
    fn test_process_parent() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let parent = process.parent();
            println!("Process parent: {:?}", parent);
        }
    }

    #[test]
    fn test_process_pid() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let pid = process.pid();
            println!("Process PID: {:?}", pid);
        }
    }

    #[test]
    fn test_process_root() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let root = process.root();
            println!("Process root: {:?}", root);
        }
    }

    #[test]
    fn test_process_run_time() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let run_time = process.run_time();
            println!("Process run time: {:?}", run_time);
        }
    }

    #[test]
    fn test_process_session_id() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let session_id = process.session_id();
            println!("Process session ID: {:?}", session_id);
        }
    }

    #[test]
    fn test_process_start_time() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let start_time = process.start_time();
            println!("Process start time: {:?}", start_time);
        }
    }

    #[test]
    fn test_process_status() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let status = process.status();
            println!("Process status: {:?}", status);
        }
    }

    #[test]
    fn test_process_tasks() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let tasks = process.tasks();
            println!("Process tasks: {:?}", tasks);
        }
    }

    #[test]
    fn test_process_thread_kind() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let thread_kind = process.thread_kind();
            println!("Process thread kind: {:?}", thread_kind);
        }
    }

    #[test]
    fn test_process_user_id() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let user_id = process.user_id();
            println!("Process user ID: {:?}", user_id);
        }
    }

    #[test]
    fn test_process_virtual_memory() {
        let system = super::refresh_processes();
        let processes = system.processes();

        for process in processes.values().take(5) {
            let virtual_memory = process.virtual_memory();
            println!("Process virtual memory: {} bytes", virtual_memory);
        }
    }

    #[test]
    fn test_process_wait() {
        // Spawn a simple sleep process (more reliable than using test binary)
        let sleep_duration_secs = 3;
        let mut child_process = if cfg!(windows) {
            std::process::Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("timeout /T {} /NOBREAK", sleep_duration_secs),
                ])
                .spawn()
        } else {
            std::process::Command::new("sleep")
                .arg(sleep_duration_secs.to_string())
                .spawn()
        }
        .expect("Failed to spawn sleep process");

        let pid = sysinfo::Pid::from_u32(child_process.id() as u32);
        println!("[TEST] Monitoring process with PID: {}", pid);

        // Find the process in sysinfo
        let system = wait_for_process_registration(pid);
        let process = system
            .process(pid)
            .expect("Process should be registered in sysinfo by now");

        // Start a separate thread that will terminate the process after a delay
        let termination_delay = std::time::Duration::from_secs(1);
        let killer_thread = std::thread::spawn(move || {
            std::thread::sleep(termination_delay);
            println!(
                "[TEST] Terminating process after {:?} delay",
                termination_delay
            );
            let _ = child_process.kill();
        });

        // Call process.wait() with diagnostics
        let start_time = std::time::Instant::now();
        let wait_result = process.wait();
        let elapsed = start_time.elapsed();

        println!(
            "[TEST] Wait completed in {:?} with result: {:?}",
            elapsed, wait_result
        );

        // Ensure the killer thread completes
        killer_thread.join().expect("Killer thread panicked");

        // Verify the process was successfully waited for
        assert!(
            wait_result.is_some(),
            "Process wait should return non-None value"
        );
        assert!(
            elapsed >= termination_delay,
            "Wait should have blocked until the process was terminated"
        );
    }

    /// Waits for a process to be registered in sysinfo with exponential backoff
    fn wait_for_process_registration(pid: sysinfo::Pid) -> sysinfo::System {
        let mut system = sysinfo::System::new();
        let mut retries = 0;
        let max_retries = 5;
        let initial_delay_ms = 20;

        loop {
            system.refresh_processes(sysinfo::ProcessesToUpdate::All, false);

            if let Some(_) = system.process(pid) {
                println!("[TEST] Process {} found after {} retries", pid, retries);
                return system;
            }

            if retries >= max_retries {
                break;
            }

            let delay = initial_delay_ms * (1 << retries); // Exponential backoff
            println!(
                "[TEST] Process {} not found, retrying in {}ms (attempt {}/{})",
                pid,
                delay,
                retries + 1,
                max_retries
            );
            std::thread::sleep(std::time::Duration::from_millis(delay));
            retries += 1;
        }

        panic!(
            "Process {} never appeared in sysinfo after {} retries",
            pid, max_retries
        );
    }
}
