mod balance;
mod configuration;
mod util;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

use crate::util::{country_from_code, paid_holidays_from_names};

#[derive(clap::Subcommand)]
enum Command {
    /// calculate leave balance on a future date
    On {
        /// future date to calculate leave for
        date: chrono::NaiveDate,
        /// file path to configuration
        config_file: std::path::PathBuf,
        /// current leave balance in hours
        #[clap(short, long)]
        balance: Option<f64>,
        /// starting date as YYYY-MM-DD (defaults to today)
        #[clap(short, long)]
        from: Option<chrono::NaiveDate>,
        /// next pay day as YYYY-MM-DD (defaults to today)
        #[clap(short, long)]
        next_pay_day: Option<chrono::NaiveDate>,
        /// balance threshold under which to warn
        #[clap(short, long)]
        warn_threshold: Option<u32>,
        #[clap(short, long)]
        verbose: bool,
    },
    /// write an empty configuration file
    Write {
        /// file path
        filename: std::path::PathBuf,
    },
    Holidays {
        year: holidays::Year,
        config_file: std::path::PathBuf,
    },
}

fn main() {
    let arguments = Cli::parse();

    match arguments.command {
        Command::On {
            date,
            config_file,
            balance,
            from,
            next_pay_day,
            warn_threshold,
            verbose,
        } => {
            let contents = std::fs::read_to_string(config_file).unwrap();
            let configuration: configuration::Configuration = toml::from_str(&contents).unwrap();

            let balance = balance::balance_on(
                date,
                configuration.policy,
                balance,
                from,
                next_pay_day,
                configuration.plans.paid.leave,
                warn_threshold,
                verbose,
            );

            println!("{:.1}", balance.num_seconds() as f64 / 3600.0);
        }
        Command::Write { filename } => {
            let configuration = configuration::Configuration::default();

            let contents = toml::to_string_pretty(&configuration).unwrap();
            std::fs::write(filename, contents.as_bytes()).unwrap();
        }
        Command::Holidays { year, config_file } => {
            let contents = std::fs::read_to_string(config_file).unwrap();
            let configuration: configuration::Configuration = toml::from_str(&contents).unwrap();

            let holidays = paid_holidays_from_names(
                configuration.policy.paid_holidays,
                country_from_code(configuration.policy.country).unwrap(),
                year,
            )
            .unwrap();

            for holiday in holidays {
                println!("{} {}", holiday.date, holiday.name);
            }
        }
    }
}
