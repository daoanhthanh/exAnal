use crate::args::Args;
use std::collections::HashMap;
use std::fs;
pub fn process(args: Args) {
    let walker = fs::read_dir(args.path).unwrap();
    let mut counts: HashMap<_, _> = HashMap::new();
    let mut total_files = 0;
}
