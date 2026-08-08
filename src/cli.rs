use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

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
    /// Extract zip archive from a URL
    ExtractZipFromUrl(ExtractZipFromUrl),
    /// Download PicoCSS
    PicoCssDownload(PicoCssDownloadArgs),
}

#[derive(Debug, Args)]
pub struct PicoCssDownloadArgs {
    ///  Pico CSS version to download
    #[arg(short, long, default_value = "2.1.1")]
    pub pico_css_version: String,

    ///  Path to extract PicoCSS files to
    #[arg(short, long, default_value = "css")]
    pub extract_path: PathBuf,
}

#[derive(Debug, Args)]
pub struct ExtractZipFromUrl {
    ///  Zip Archive URL
    #[arg(short, long)]
    pub url: String,

    /// Path to extract zip archive to
    #[arg(short, long)]
    pub extract_path: PathBuf,
}
