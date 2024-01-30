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
    /// calculate leave balance on a future date
    On {
        /// future date to calculate leave for
        date: chrono::NaiveDate,
        /// file path to configuration
        config_file: std::path::PathBuf,
        /// current leave balance in hours
        current_leave_balance: Option<f64>,
        /// next pay day in YYYY-MM-DD format
        next_pay_day: Option<chrono::NaiveDate>,
        /// balance threshold under which to warn
        balance_warn_threshold: Option<u32>,
        #[clap(short, long)]
        verbose: bool,
    },
    /// write an empty configuration file
    Write {
        /// file path
        filename: std::path::PathBuf,
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
            let contents = std::fs::read_to_string(configuration).unwrap();
            let configuration: configuration::Configuration = toml::from_str(&contents).unwrap();

            let balance = balance::balance_on(
                on,
                configuration.policy,
                next_pay_day,
                current_leave_balance,
                configuration.plans.paid.leave,
                balance_warn_threshold,
                verbose,
            );

            println!("{:.1}", balance.num_seconds() as f64 / 3600.0);
        }
        Command::Write { filename } => {
            let configuration = configuration::Configuration::default();

            let contents = toml::to_string_pretty(&configuration).unwrap();
            std::fs::write(filename, contents.as_bytes()).unwrap();
        }
    }
}
