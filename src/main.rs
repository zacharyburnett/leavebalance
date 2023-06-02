mod balance;
mod configuration;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // future date to calculate leave for
    on: chrono::NaiveDate,
    // path to configuration file
    configuration: std::path::PathBuf,
    // current leave balance in hours
    current_leave_balance: Option<f64>,
    // next pay day
    next_pay_day: Option<chrono::NaiveDate>,
    // balance threshold under which to warn
    balance_warn_threshold: Option<u32>,
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let arguments = Cli::parse();

    let file = std::fs::File::open(arguments.configuration).unwrap();
    let configuration: configuration::Configuration = serde_yaml::from_reader(file).unwrap();

    let working_time = configuration.policy.working_hours.1 - configuration.policy.working_hours.0;

    let balance = balance::balance_on(
        arguments.on,
        configuration.policy,
        arguments.next_pay_day,
        arguments.current_leave_balance,
        configuration.planned_leave,
        arguments.balance_warn_threshold,
        arguments.verbose,
    );

    println!(
        "your leave balance will be {:} hours ({:} working days) on {:}",
        balance.num_hours(),
        balance.num_hours() / working_time.num_hours(),
        arguments.on,
    );
}
