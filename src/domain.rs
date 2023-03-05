mod args;
use args::Args;
pub fn process(args: Args) {
    print!("Hello {}!", args.name)
}
