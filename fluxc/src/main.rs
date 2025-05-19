use anyhow::{Context, Result};
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
        #[arg(short, long, value_name = "FILE")]
        output: Option<String>,
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

impl From<BackendOpt> for flux_lang::codegen::Backend {
    fn from(opt: BackendOpt) -> Self {
        match opt {
            BackendOpt::Llvm => flux_lang::codegen::Backend::Llvm,
            BackendOpt::Cranelift => flux_lang::codegen::Backend::Cranelift,
            BackendOpt::Wasm => flux_lang::codegen::Backend::Wasm,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile {
            input,
            output,
            backend,
        } => {
            let source = fs::read_to_string(&input)
                .with_context(|| format!("failed to read input `{}`", input))?;
            flux_lang::compile_with_backend(&source, backend.into()).context("compile error")?;
            if let Some(path) = output {
                println!("would write output to {path}");
            }
        }
        Commands::Ast { input } => {
            let source = fs::read_to_string(&input)
                .with_context(|| format!("failed to read input `{}`", input))?;
            let ast = flux_lang::parse_program(&source)?;
            println!("{ast:#?}");
        }
        Commands::Repl => {
            println!("REPL stub");
        }
    }
    Ok(())
}
