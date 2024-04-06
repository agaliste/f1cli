use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum DriverCommands {
    Info { #[clap(long, value_parser = crate::utils::year_validator)] year: i32 },
}

// Driver command handlers go here
