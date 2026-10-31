use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long)]
    output: PathBuf,
}
