pub mod battery_helper;
pub mod sys_info_helper;

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
