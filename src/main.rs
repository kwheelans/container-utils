use crate::download::download_css_archive;
use clap::Parser;
use tracing::error;
use tracing::level_filters::LevelFilter;
use crate::cli::CliArgs;

mod download;
mod cli;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();
    let cli = CliArgs::parse();

    if cli.pico_css_download {
        if let Err(error) = download_css_archive() {
            error!("{}", error)
        }
    }
}
