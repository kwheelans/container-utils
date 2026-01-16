use crate::download::download_css_archive;
use std::process::ExitCode;
use tracing::error;
use tracing::level_filters::LevelFilter;

mod download;
mod cli;

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    match download_css_archive() {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            error!("{}", error);
            ExitCode::FAILURE
        }
    }
}
