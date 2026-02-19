use image_processor::args;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();
    image_processor::process_image(args)?;
    Ok(())
}
