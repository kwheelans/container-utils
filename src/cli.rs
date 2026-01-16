use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub commands: Commands,

    /// Display more verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Download PicoCSS
    PicoCssDownload(PicoCssDownloadArgs)
}

#[derive(Debug, Args)]
pub struct PicoCssDownloadArgs {
    ///  Pico CSS version to download
    #[arg(short, long, default_value = "2.1.1")]
    pub pico_css_version: String,

    ///  Pico CSS version to download
    #[arg(short, long, default_value = "css")]
    pub output_path: PathBuf,
}
