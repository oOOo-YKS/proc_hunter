pub fn get_battery_brief_info() -> Option<(f32, f32)> {
    let mut sum_energy = 0.0;
    let mut sum_energy_full = 0.0;
    let manager = battery::Manager::new();
    let manager = match manager {
        Ok(manager) => manager,
        Err(_) => return None,
    };
    let batteries_r = manager.batteries();
    let batteries = match batteries_r {
        Ok(batteries) => batteries,
        Err(_) => return None,
    };
    for battery_r in batteries.into_iter().collect::<Vec<_>>() {
        let battery = match battery_r {
            Ok(battery) => battery,
            Err(_) => return None,
        };
        let energy = battery.energy().value;
        let energy_full = battery.energy_full().value;
        sum_energy += energy;
        sum_energy_full += energy_full;
    }
    return Some((sum_energy, sum_energy_full));
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_serial_number() {
        let manager = battery::Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        for battery_r in batteries.collect::<Vec<_>>() {
            let battery = battery_r.unwrap();
            let serial_number = battery.serial_number().unwrap_or("Unknown");
            println!("Battery Serial Number: {}", serial_number);
        }
    }

    #[test]
    fn test_cycle_count() {
        let manager = battery::Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        for battery_r in batteries.collect::<Vec<_>>() {
            let battery = battery_r.unwrap();
            let cycle_count = battery.cycle_count().unwrap();
            println!(
                "Battery: {} - Cycle Count: {}",
                battery.serial_number().unwrap_or("Unknown"),
                cycle_count
            );
        }
    }

    #[test]
    fn test_energy() {
        let manager = battery::Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        for battery_r in batteries.collect::<Vec<_>>() {
            let battery = battery_r.unwrap();
            let energy = battery.energy().value;
            println!(
                "Battery: {} - Energy: {:.2} Wh",
                battery.serial_number().unwrap_or("Unknown"),
                energy
            );
        }
    }

    #[test]
    fn test_energy_full() {
        let manager = battery::Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        for battery_r in batteries.collect::<Vec<_>>() {
            let battery = battery_r.unwrap();
            let energy_full = battery.energy_full().value;
            println!(
                "Battery: {} - Energy Full: {:.2} Wh",
                battery.serial_number().unwrap_or("Unknown"),
                energy_full
            );
        }
    }

    #[test]
    fn test_energy_full_design() {
        let manager = battery::Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        for battery_r in batteries.collect::<Vec<_>>() {
            let battery = battery_r.unwrap();
            let energy_full_design = battery.energy_full_design().value;
            println!(
                "Battery: {} - Energy Full Design: {:.2} Wh",
                battery.serial_number().unwrap_or("Unknown"),
                energy_full_design
            );
        }
    }

    #[test]
    fn test_energy_rate() {
        let manager = battery::Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        for battery_r in batteries.collect::<Vec<_>>() {
            let battery = battery_r.unwrap();
            let energy_rate = battery.energy_rate().value;
            println!(
                "Battery: {} - Energy Rate: {:.2} W",
                battery.serial_number().unwrap_or("Unknown"),
                energy_rate
            );
        }
    }

    #[test]
    fn test_model() {
        let manager = battery::Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        for battery_r in batteries.collect::<Vec<_>>() {
            let battery = battery_r.unwrap();
            let model = battery.model().unwrap_or("Unknown");
            println!("Battery Model: {}", model);
        }
    }

    #[test]
    fn test_state() {
        let manager = battery::Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        for battery_r in batteries.collect::<Vec<_>>() {
            let battery = battery_r.unwrap();
            let state = battery.state();
            println!(
                "Battery: {} - State: {:?}",
                battery.serial_number().unwrap_or("Unknown"),
                state
            );
        }
    }

    #[test]
    fn test_state_of_charge() {
        let manager = battery::Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        for battery_r in batteries.collect::<Vec<_>>() {
            let battery = battery_r.unwrap();
            let ratio = battery.state_of_charge().value;
            println!(
                "Battery: {} - State of Charge: {:.2}%",
                battery.serial_number().unwrap_or("Unknown"),
                ratio * 100.0
            );
        }
    }

    #[test]
    fn test_state_of_health() {
        let manager = battery::Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        for battery_r in batteries.collect::<Vec<_>>() {
            let battery = battery_r.unwrap();
            let state_of_health = battery.state_of_health().value;
            println!(
                "Battery: {} - State of Health: {:.2}%",
                battery.serial_number().unwrap_or("Unknown"),
                state_of_health * 100.0
            );
        }
    }

    #[test]
    fn test_technology() {
        let manager = battery::Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        for battery_r in batteries.collect::<Vec<_>>() {
            let battery = battery_r.unwrap();
            let technology = battery.technology();
            println!("Battery Technology: {}", technology);
        }
    }

    #[test]
    fn test_vendor() {
        let manager = battery::Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        for battery_r in batteries.collect::<Vec<_>>() {
            let battery = battery_r.unwrap();
            let vendor = battery.vendor().unwrap_or("Unknown");
            println!("Battery Vendor: {}", vendor);
        }
    }

    #[test]
    fn test_voltage() {
        let manager = battery::Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        for battery_r in batteries.collect::<Vec<_>>() {
            let battery = battery_r.unwrap();
            let voltage = battery.voltage().value;
            println!(
                "Battery: {} - Voltage: {:.2} V",
                battery.serial_number().unwrap_or("Unknown"),
                voltage
            );
        }
    }
}
