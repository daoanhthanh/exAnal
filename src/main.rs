pub(crate) mod args;
mod domain;
use args::Args;
use clap::Parser;
use domain::process;

fn main() {
    let args = Args::parse();
    process(args);
}
