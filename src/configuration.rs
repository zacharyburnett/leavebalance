#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Configuration {
    pub policy: LeavePolicy,
    #[serde(default)]
    pub planned_leave: Vec<(chrono::NaiveDate, chrono::NaiveDate)>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LeavePolicy {
    pub working_hours: (chrono::NaiveTime, chrono::NaiveTime),
    pub working_days: Vec<chrono::Weekday>,
    pub days_in_pay_period: u32,
    pub hours_accrued_per_pay_period: f64,
}

impl Default for LeavePolicy {
    fn default() -> Self {
        Self {
            working_hours: (
                chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            ),
            working_days: vec![
                chrono::Weekday::Mon,
                chrono::Weekday::Tue,
                chrono::Weekday::Wed,
                chrono::Weekday::Thu,
                chrono::Weekday::Fri,
            ],
            days_in_pay_period: 14,
            hours_accrued_per_pay_period: 0.0,
        }
    }
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

        assert_eq!(
            configuration.policy.working_hours,
            (
                chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap()
            )
        );
    }

    #[test]
    fn test_example_2() {
        let path = format!(
            "{:}/{:}",
            env!("CARGO_MANIFEST_DIR"),
            "examples/example_2.yaml"
        );

        let file = std::fs::File::open(path).unwrap();
        let configuration: Configuration = serde_yaml::from_reader(file).unwrap();

        assert_eq!(
            configuration.policy.working_hours,
            (
                chrono::NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                chrono::NaiveTime::from_hms_opt(16, 0, 0).unwrap()
            )
        );
    }
}
