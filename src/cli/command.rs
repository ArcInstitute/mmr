use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    },
    Parser,
};

// Configures Clap v3-style help menu colors
const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

use super::{IndexOptions, IoOptions, MappingOptions, RunOptions};

#[derive(Parser)]
#[command(styles = STYLES)]
pub struct Cli {
    #[clap(flatten)]
    pub io_options: IoOptions,

    #[clap(flatten)]
    pub run_options: RunOptions,

    #[clap(flatten)]
    pub index_options: IndexOptions,

    #[clap(flatten)]
    pub mapping_options: MappingOptions,
}
