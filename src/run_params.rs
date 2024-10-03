use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author)]
pub struct Args {
    #[arg(short = 'c', long, default_value = "")]
    pub config: String,
}

pub fn get_args() -> Args {
    Args::parse()
}