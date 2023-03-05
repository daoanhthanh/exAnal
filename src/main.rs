mod args;
mod domain;
use domain::process;
use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    print!("Hello {}!", args.name)
}
