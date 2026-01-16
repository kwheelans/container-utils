use crate::cli::{CliArgs, Commands};
use crate::download::download_css_archive;
use clap::Parser;
use tracing::error;
use tracing::level_filters::LevelFilter;

mod cli;
mod download;

fn main() {
    let cli = CliArgs::parse();
    let log_level = match cli.verbose {
        true => LevelFilter::DEBUG,
        false => LevelFilter::INFO,
    };
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .init();
    let cli = CliArgs::parse();

    match cli.commands {
        Commands::PicoCssDownload(args) => {
            if let Err(error) = download_css_archive(args.pico_css_version.as_str(), args.output_path) {
                error!("{}", error)
            }
        }
    }
}
