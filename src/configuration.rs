use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Configuration {
    pub policy: LeavePolicy,
    #[serde(default)]
    pub plans: LeavePlans,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LeavePolicy {
    pub work_week: HashMap<chrono::Weekday, (chrono::NaiveTime, chrono::NaiveTime)>,
    pub days_in_pay_period: u32,
    pub hours_accrued_per_pay_period: f64,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct LeavePlans {
    pub paid_leave: Vec<(chrono::NaiveDate, chrono::NaiveDate)>,
}

impl Default for LeavePolicy {
    fn default() -> Self {
        let mut work_week = HashMap::new();
        work_week.insert(
            chrono::Weekday::Mon,
            (
                chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            ),
        );
        work_week.insert(
            chrono::Weekday::Tue,
            (
                chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            ),
        );
        work_week.insert(
            chrono::Weekday::Wed,
            (
                chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            ),
        );
        work_week.insert(
            chrono::Weekday::Thu,
            (
                chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            ),
        );
        work_week.insert(
            chrono::Weekday::Fri,
            (
                chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            ),
        );

        Self {
            work_week,
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
            "examples/example_1.toml"
        );

        let contents = std::fs::read_to_string(path).unwrap();
        let configuration: Configuration = toml::from_str(&contents).unwrap();

        assert_eq!(
            configuration.policy.work_week[&chrono::Weekday::Mon],
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
            "examples/example_2.toml"
        );

        let contents = std::fs::read_to_string(path).unwrap();
        let configuration: Configuration = toml::from_str(&contents).unwrap();

        assert_eq!(
            configuration.policy.work_week[&chrono::Weekday::Mon],
            (
                chrono::NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                chrono::NaiveTime::from_hms_opt(16, 0, 0).unwrap()
            )
        );
    }
}
