use clap::Parser;

/// FluxLang compiler CLI
#[derive(Parser)]
struct Cli {
    /// Input file
    #[arg(value_name = "FILE")]
    input: String,
}

fn main() {
    let _args = Cli::parse();
    println!("fluxc stub");
}
