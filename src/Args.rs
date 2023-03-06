use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "Edward Dao. <daoanhthanh.work@gmail.com>", 
    version, 
    about = "Counts the percentage of each file extension in a folder", 
    long_about = None
)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pub path: String,

    /// Analyze subdirectories recursively
    #[arg(short, long, default_value = "false")]
    pub recursive: bool,
}
