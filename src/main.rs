use clap::{Args, Parser, Subcommand};
use proc_hunter::models_fn_packer::battery_helper::get_battery_brief_info;

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
    // if is not none, turn all to false
    #[arg(long, default_value_t = false)]
    battery: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info(args) => {
            let mut all = true; // Default value for all
            match args.battery {
                true => {
                    all = false; // Set all to false if battery is true
                    let (sum_energy, sum_energy_full) = get_battery_brief_info().unwrap();
                    println!("Battery sum energy: {}", sum_energy);
                    println!("Battery sum energy full: {}", sum_energy_full);
                }
                false => {}
            }
            if all {
                // Call the function to get all information
                println!("All information requested");
            }
        }
    }
}
