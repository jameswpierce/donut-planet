use clap::{Parser, Subcommand};

mod build;
mod config;
mod serve;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // builds cardamon's static site
    Build {},
    // starts the server
    Serve {},
    // starts server in dev mode (rebuilds with changes to templates folder)
    Dev {},
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("donut-planet v{}", env!("CARGO_PKG_VERSION"));
    println!("by: burgerdog");

    match &cli.command {
        Some(Commands::Build {}) => {
            println!("building donut-planet...");
            build::run()?;
        }
        Some(Commands::Serve {}) => {
            println!("building donut-planet...");
            build::run()?;
            println!("starting donut-planet server...");
            serve::run(false)?;
        }
        Some(Commands::Dev {}) => {
            println!("building donut-planet...");
            build::run()?;
            println!("starting donut-planet server in dev mode...");
            serve::run(true)?;
        }
        None => {}
    }

    println!("welcome to donut-planet");

    Ok(())
}
