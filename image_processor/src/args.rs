use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub output: String,
    #[arg(short, long)]
    pub plugin: String,
    #[arg(long)]
    pub params: String,
    #[arg(long, default_value = "target/debug")]
    pub plugin_path: String,
}
