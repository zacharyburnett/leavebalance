mod balance;
mod configuration;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    On {
        // future date to calculate leave for
        date: chrono::NaiveDate,
        // path to configuration file (create one with ```leavebalance config`)
        config_file: std::path::PathBuf,
        // current leave balance in hours
        current_leave_balance: Option<f64>,
        // next pay day
        next_pay_day: Option<chrono::NaiveDate>,
        // balance threshold under which to warn
        balance_warn_threshold: Option<u32>,
        #[clap(short, long)]
        verbose: bool,
    },
    Config {
        // path to configuration file to write
        filename: std::path::PathBuf,
        hours_accrued_per_pay_period: f64,
    },
}

fn main() {
    let arguments = Cli::parse();

    match arguments.command {
        Command::On {
            date: on,
            config_file: configuration,
            current_leave_balance,
            next_pay_day,
            balance_warn_threshold,
            verbose,
        } => {
            let file = std::fs::File::open(configuration).unwrap();
            let configuration: configuration::Configuration =
                serde_yaml::from_reader(file).unwrap();

            let working_time =
                configuration.policy.working_hours.1 - configuration.policy.working_hours.0;

            let balance = balance::balance_on(
                on,
                configuration.policy,
                next_pay_day,
                current_leave_balance,
                configuration.planned_leave,
                balance_warn_threshold,
                verbose,
            );

            println!(
                "your leave balance will be {:} hours ({:} working days) on {:}",
                balance.num_hours(),
                balance.num_hours() / working_time.num_hours(),
                on,
            );
        }
        Command::Config {
            hours_accrued_per_pay_period,
            filename,
        } => {
            let mut configuration = configuration::Configuration::default();
            configuration.policy.hours_accrued_per_pay_period = hours_accrued_per_pay_period;

            let file = std::fs::File::create(filename).unwrap();
            match serde_yaml::to_writer(file, &configuration) {
                Ok(_) => {}
                Err(error) => {
                    panic!("{:}", error);
                }
            }
        }
    }
}
