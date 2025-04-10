mod battery_helper;
mod sys_info_helper;

pub use battery_helper::get_battery_brief_info;
pub use sys_info_helper::get_cpu_brief_info;
pub use sys_info_helper::get_cpu_brief_info_with_1_ms_sleep;
pub use sys_info_helper::get_memory_brief_info;
pub use sys_info_helper::get_processes_brief_info;

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_battery_brief_info() {
        let result = crate::models_fn_packer::battery_helper::get_battery_brief_info();
        assert!(result.is_some(), "Failed to get battery brief info");
        let (sum_energy, sum_energy_full) = result.unwrap();
        assert!(sum_energy >= 0.0, "Sum energy should be non-negative");
        assert!(
            sum_energy_full >= 0.0,
            "Sum energy full should be non-negative"
        );
    }
}
