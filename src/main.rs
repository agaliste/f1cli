mod cli;
mod commands;
mod utils;

use clap::Parser;
use commands::session::handle_session_command;

fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Session { command } => handle_session_command(command),
        _ => println!("Other commands to be handled"),
    }
    
}
