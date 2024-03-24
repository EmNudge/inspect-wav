use std::path::Path;

use clap::Parser;
use color_eyre::eyre::{eyre, Result};

#[derive(Parser, Debug)]
#[command(about = "WAV file inspector")]
pub struct Args {
  #[arg(short, long, help = "The WAV file to inspect")]
  pub file: String,
  
  #[arg(short, long, help = "Whether to use a TUI", default_value = "false")]
  pub tui: bool,
}

pub fn get_args() -> Result<Args> {
  let args = Args::parse();

  if !Path::new(&args.file).exists() {
    return Err(eyre!("File \"{}\" does not exist", &args.file));
  }

  if !args.file.ends_with("wav") && !args.file.ends_with("wav") {
    return Err(eyre!("File \"{}\" may not be a WAV file", &args.file));
  }

  Ok(args)
}