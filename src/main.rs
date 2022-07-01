extern crate log;

use clap::{Parser, Subcommand};

mod cmd;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Init sample site project
    Init {},
    /// Build site
    Build(cmd::BuildArgs),
    /// Serve site and watch for changes
    Serve(cmd::ServerArgs),
}

fn main() {
    // set logger
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::max())
        .format_module_path(false)
        .init();

    // parse cli arguments
    let args = Cli::parse();
    // run command
    match args.command {
        Commands::Init {} => {
            println!("Init command");
        }
        Commands::Build(args) => {
            println!("Build command");
            println!("{:?}", args);
        }
        Commands::Serve(args) => {
            println!("Serve command");
            println!("{:?}", args);
        }
    }
}