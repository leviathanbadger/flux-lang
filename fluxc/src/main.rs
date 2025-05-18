use clap::{Parser, Subcommand};
use std::fs;

/// FluxLang compiler CLI
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a FluxLang source file
    Compile {
        #[arg(value_name = "FILE")]
        input: String,
    },
    /// Interactive REPL (not yet implemented)
    Repl,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { input } => {
            let source = fs::read_to_string(&input).expect("failed to read input");
            if let Err(e) = flux_lang::compile(&source) {
                eprintln!("compile error: {e}");
            }
        }
        Commands::Repl => {
            println!("REPL stub");
        }
    }
}
