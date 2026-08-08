use crate::cli::{CliArgs, Commands};
use crate::css::download_css_archive;
use crate::zip_archive::{download_zip_archive, extract_zip_archive};
use clap::Parser;
use tracing::error;
use tracing::level_filters::LevelFilter;

mod cli;
mod css;
mod zip_archive;

fn main() {
    let cli = CliArgs::parse();
    let log_level = match cli.verbose {
        true => LevelFilter::DEBUG,
        false => LevelFilter::INFO,
    };
    tracing_subscriber::fmt().with_max_level(log_level).init();

    if let Err(error) = run_command(cli) {
        error!("{}", error)
    }
}

fn run_command(cli: CliArgs) -> Result<(), anyhow::Error> {
    match cli.commands {
        Commands::PicoCssDownload(args) => {
            download_css_archive(args.pico_css_version.as_str(), args.extract_path)?;
        }
        Commands::ExtractZipFromUrl(args) => {
            let archive = download_zip_archive(args.url)?;
            extract_zip_archive(archive, args.extract_path)?;
        }
    }
    Ok(())
}
