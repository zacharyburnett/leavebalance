use chrono::Datelike;

pub fn balance_on(
    on: chrono::NaiveDate,
    policy: crate::configuration::LeavePolicy,
    next_pay_day: Option<chrono::NaiveDate>,
    starting_balance: Option<f64>,
    planned_paid_leave: Vec<(chrono::NaiveDate, chrono::NaiveDate)>,
    balance_warn_threshold: Option<u32>,
    verbose: bool,
) -> chrono::Duration {
    let mut balance = starting_balance.unwrap_or(0.0);
    let balance_warn_threshold = balance_warn_threshold.unwrap_or(0);

    let today = chrono::Local::now().date_naive();
    let mut next_pay_day = next_pay_day.unwrap_or(today);

    let mut total_used = 0.0;
    let mut total_accrued = 0.0;

    for future_date in DateRange(today, on) {
        // check if holiday
        for holiday in &policy.paid_holidays {
            if &future_date == holiday {
                break;
            }
        }

        for (work_day, hours) in &policy.work_week {
            if work_day == &future_date.weekday() {
                let working_time = hours.1 - hours.0;

                if future_date >= next_pay_day {
                    let accrued = policy.hours_accrued_per_pay_period;
                    balance += accrued;

                    if verbose {
                        total_accrued += accrued;
                        println!("{:} +{:.1}h => {:.1}h", future_date, accrued, balance);
                    }

                    next_pay_day += chrono::Duration::days(policy.days_in_pay_period as i64);
                }

                for planned_paid_leave in &planned_paid_leave {
                    if future_date > planned_paid_leave.0 && future_date < planned_paid_leave.1 {
                        for leave_date in DateRange(planned_paid_leave.0, planned_paid_leave.1) {
                            if leave_date == future_date {
                                let used = working_time.num_seconds() as f64 / 3600.0;
                                balance -= used;

                                if verbose {
                                    total_used += used;
                                    println!("{:} -{:.1}h => {:.1}h", future_date, used, balance);
                                }

                                if balance < balance_warn_threshold as f64 {
                                    eprintln!(
                                    "your planned leave on {:} would deplete your leave balance to {:.1}h!",
                                    leave_date, balance
                                );
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    if verbose {
        println!("accrued {:.1}h and used {:.1}h", total_accrued, total_used,);
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
