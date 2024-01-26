mod user_actions;
mod tamaowlchi;
mod constants;
mod config_file;

use std::process;
use clap::{Parser, Subcommand};
use config_file::{load_current_tamaowlchi, save_tamaowlchi};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    /// Create a new tamaowlchi. Only if you don't have any tamaowlchi alive
    New,
    /// Remove all your tamaowlchi poops
    Clean,
    /// Restore the food of your tamaowlchi to 50
    Feed,
    /// Restore the heal of your tamaowlchi to 50
    Heal,
    /// Put your tamaowlchi to bed
    Sleep,
    /// Wake up your tamaowlchi
    Wake,
    /// Get your tamaowlchi status
    Status,
}


fn main() {
    let cli = Cli::parse();

    let mut tamaowlchi_opt = load_current_tamaowlchi();

    if tamaowlchi_opt.is_none()  {
        if let Commands::New = &cli.command { 
            tamaowlchi_opt = Some(user_actions::create_new_tamaowlchi());
        } else {
            eprintln!("You don't have any tamaowlchi. Use 'tamaowlchi new' to get a new one !");
            process::exit(1)
        }
    }

    
    let mut tamaowlchi = tamaowlchi_opt.unwrap();

    match &cli.command {
        Commands::New => user_actions::status(&tamaowlchi),
        Commands::Clean => user_actions::clean(&mut tamaowlchi),
        Commands::Feed => user_actions::feed(&mut tamaowlchi),
        Commands::Heal => user_actions::heal(&mut tamaowlchi),
        Commands::Sleep => user_actions::put_to_bed(&mut tamaowlchi),
        Commands::Wake => user_actions::wake_up(&mut tamaowlchi),
        Commands::Status => user_actions::status(&tamaowlchi),
    }

    save_tamaowlchi(&tamaowlchi);
}

