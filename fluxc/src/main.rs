use clap::{Parser, Subcommand, ValueEnum};
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
        #[arg(long, value_enum, default_value = "llvm")]
        backend: BackendOpt,
    },
    /// Parse a FluxLang file and dump the AST
    Ast {
        #[arg(value_name = "FILE")]
        input: String,
    },
    /// Interactive REPL (not yet implemented)
    Repl,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum BackendOpt {
    Llvm,
    Cranelift,
    Wasm,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { input, backend } => {
            let source = fs::read_to_string(&input).expect("failed to read input");
            let backend = match backend {
                BackendOpt::Llvm => flux_lang::codegen::Backend::Llvm,
                BackendOpt::Cranelift => flux_lang::codegen::Backend::Cranelift,
                BackendOpt::Wasm => flux_lang::codegen::Backend::Wasm,
            };
            if let Err(e) = flux_lang::compile_with_backend(&source, backend) {
                eprintln!("compile error: {e}");
            }
        }
        Commands::Ast { input } => {
            let source = fs::read_to_string(&input).expect("failed to read input");
            match flux_lang::parse_program(&source) {
                Ok(ast) => println!("{ast:#?}"),
                Err(e) => eprintln!("parse error: {e}"),
            }
        }
        Commands::Repl => {
            println!("REPL stub");
        }
    }
}
