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
                configuration.plans.paid_leave,
                balance_warn_threshold,
                verbose,
            );

            println!(
                "your leave balance will be {:.1}h on {:}",
                balance.num_seconds() as f64 / 3600.0,
                on,
            );
        }
        Command::Config { filename } => {
            let configuration = configuration::Configuration::default();

            let contents = toml::to_string_pretty(&configuration).unwrap();
            std::fs::write(filename, contents.as_bytes()).unwrap();
        }
    }
}
