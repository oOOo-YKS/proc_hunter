use clap::{Args, Parser, Subcommand};
use proc_hunter::models_fn_packer::get_battery_brief_info;
use proc_hunter::models_fn_packer::get_cpu_brief_info_with_1_ms_sleep;
use proc_hunter::models_fn_packer::get_memory_brief_info;
use proc_hunter::models_fn_packer::get_processes_brief_info;

#[derive(Parser)]
#[command(name = "proc_hunter")]
#[command(version = "1.0")]
#[command(about = "A useless process hunter", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get brief information about the computer
    Info(InfoArgs),
}

#[derive(Args)]
struct InfoArgs {
    #[arg(long, default_value_t = false)]
    battery: bool,

    #[arg(long, default_value_t = false)]
    cpu: bool,

    #[arg(long, default_value_t = false)]
    memory: bool,

    #[arg(long, default_value_t = false)]
    process: bool,
}

fn print_battery_brief_info() {
    let (sum_energy, sum_energy_full) = get_battery_brief_info().unwrap();
    println!("Battery sum energy: {}", sum_energy);
    println!("Battery sum energy full: {}", sum_energy_full);
    println!("");
}

fn print_cpu_brief_info() {
    let (physical_num, cpu_num, global_usage, cpus) = get_cpu_brief_info_with_1_ms_sleep();
    println!("{} cpus on {} physical core", cpu_num, physical_num);
    println!("Global CPU usage: {}", global_usage);
    println!("");
    for cpu in cpus {
        // cpu: (cpu_brand, cpu_name, cpu_frequency, cpu_usage)
        println!("{} ({}):", cpu.1, cpu.0);
        println!("   frequency: {}", cpu.2);
        println!("   usage: {}", cpu.3);
        println!("");
    }
}

fn print_memory_brief_info() {
    let (total_memory, free_memory, used_memory, total_swap, free_swap, used_swap) =
        get_memory_brief_info();
    println!("Total memory: {}bytes", total_memory);
    println!(
        "Free memory: {}bytes ({:.2}%)",
        free_memory,
        (free_memory as f32 / total_memory as f32) * 100.0
    );
    println!(
        "Used memory: {}bytes ({:.2}%)",
        used_memory,
        (used_memory as f32 / total_memory as f32) * 100.0
    );
    println!("");
    println!("Total swap: {}bytes", total_swap);
    println!(
        "Free swap: {}bytes ({:.2}%)",
        free_swap,
        (free_swap as f32 / total_swap as f32) * 100.0
    );
    println!(
        "Used swap: {}bytes ({:.2}%)",
        used_swap,
        (used_swap as f32 / total_swap as f32) * 100.0
    );
    println!("");
}

fn print_process_brief_info() {
    let (proccess_num, root) = get_processes_brief_info();
    println!("Process number: {}", proccess_num);
    println!("Root process: {}", root);
    println!("");
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info(args) => {
            let mut all = true; // Default value for all
            match args.battery {
                true => {
                    all = false; // Set all to false if battery is true
                    print_battery_brief_info();
                }
                false => {}
            }
            match args.cpu {
                true => {
                    all = false; // Set all to false if cpu is true
                    print_cpu_brief_info();
                }
                false => {}
            }
            match args.memory {
                true => {
                    all = false; // Set all to false if memory is true
                    print_memory_brief_info();
                }
                false => {}
            }
            match args.process {
                true => {
                    all = false; // Set all to false if process is true
                    print_process_brief_info();
                }
                false => {}
            }
            if all {
                print_battery_brief_info();
                print_cpu_brief_info();
                print_memory_brief_info();
                print_process_brief_info();
            }
        }
    }
}
