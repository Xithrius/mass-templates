#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::future_not_send)]

use clap::Parser;
use color_eyre::eyre::{Error, Result, WrapErr};
use handlers::args::Cli;

mod handlers;
mod utils;

use crate::handlers::config::CompleteConfig;

fn main() -> Result<(), Error> {
    color_eyre::install().unwrap();

    let mut _config = CompleteConfig::new(Cli::parse())
        .wrap_err("Unable to read configuration file.")
        .unwrap();

    std::process::exit(0)
}
