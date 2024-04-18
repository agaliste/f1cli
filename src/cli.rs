use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Session {
        #[clap(subcommand)]
        command: super::commands::SessionCommands,
    },
    // Driver {
    //     #[clap(subcommand)]
    //     command: super::commands::DriverCommands,
    // },
}
