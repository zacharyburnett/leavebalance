use chrono::Datelike;

pub fn balance_on(
    on: chrono::NaiveDate,
    policy: crate::configuration::LeavePolicy,
    next_pay_day: Option<chrono::NaiveDate>,
    starting_balance: Option<f64>,
    planned_leave: Vec<(chrono::NaiveDate, chrono::NaiveDate)>,
    balance_warn_threshold: Option<u32>,
    verbose: bool,
) -> chrono::Duration {
    let mut balance = starting_balance.unwrap_or(0.0);
    let balance_warn_threshold = balance_warn_threshold.unwrap_or(0);

    let today = chrono::Local::now().date_naive();
    let mut next_pay_day = next_pay_day.unwrap_or(today);

    let mut total_used = 0.0;
    let mut total_accrued = 0.0;

    let working_time = policy.working_hours.1 - policy.working_hours.0;
    for future_date in DateRange(today, on) {
        if policy.working_days.contains(&future_date.weekday()) {
            if future_date >= next_pay_day {
                let accrued = policy.hours_accrued_per_pay_period;

                if verbose {
                    println!("{:} - {:} hours accrued", future_date, accrued);
                    total_accrued += accrued;
                }

                balance += accrued;
                next_pay_day += chrono::Duration::days(policy.days_in_pay_period as i64);
            }

            for leave in &planned_leave {
                if future_date > leave.0 && future_date < leave.1 {
                    for leave_date in DateRange(leave.0, leave.1) {
                        if leave_date == future_date {
                            let used = working_time.num_seconds() as f64 / 3600.0;
                            balance -= used;

                            if verbose {
                                println!("{:} - {:} hours used ", future_date, used);
                                total_used += used;
                            }

                            if balance < balance_warn_threshold as f64 {
                                eprintln!(
                                    "{:} - your planned leave on {:} would deplete your leave balance to {:} hours!",
                                    leave_date, leave_date, balance
                                );
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    if verbose {
        println!(
            "accrued {:} hours ({:} working days) and used {:} hours ({:} working days)",
            total_accrued,
            total_accrued / working_time.num_hours() as f64,
            total_used,
            total_used / working_time.num_hours() as f64,
        );
    }

    chrono::Duration::seconds((balance * 3600.0) as i64)
}

struct DateRange(chrono::NaiveDate, chrono::NaiveDate);

impl Iterator for DateRange {
    type Item = chrono::NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + chrono::Duration::days(1);
            Some(std::mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}
