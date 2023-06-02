#[derive(serde::Deserialize)]
pub struct Configuration {
    pub policy: LeavePolicy,
    #[serde(default)]
    pub planned_leave: Vec<(chrono::NaiveDate, chrono::NaiveDate)>,
}

#[derive(serde::Deserialize)]
pub struct LeavePolicy {
    pub working_hours: (chrono::NaiveTime, chrono::NaiveTime),
    pub working_days: Vec<chrono::Weekday>,
    pub days_in_pay_period: u32,
    pub hours_accrued_per_pay_period: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let path = format!(
            "{:}/{:}",
            env!("CARGO_MANIFEST_DIR"),
            "examples/example_1.yaml"
        );

        let file = std::fs::File::open(path).unwrap();
        let configuration: Configuration = serde_yaml::from_reader(file).unwrap();

        assert_eq!(configuration.policy.working_day_hours, 8.0);
    }
}
