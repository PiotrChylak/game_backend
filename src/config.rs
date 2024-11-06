use url::Url;
use clap::{arg, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, short, env)]
    pub url: Url,
    #[arg(long, short, env)]
    pub sender_address: String,
    #[arg(long, short, env)]
    pub private_key: String,
}
