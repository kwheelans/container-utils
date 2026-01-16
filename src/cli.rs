use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Download Pico CSS archive and exit
    #[arg(short, long, conflicts_with = "config")]
    pub pico_css_download: bool,
}
