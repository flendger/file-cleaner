use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author)]
pub struct Args {
    #[arg(short = 'c', long, default_value = "")]
    pub config: String,
    #[arg(short = 'l', long, default_value = "", name = "LOG_FILE")]
    pub log: String,
}

pub fn get_args() -> Args {
    Args::parse()
}