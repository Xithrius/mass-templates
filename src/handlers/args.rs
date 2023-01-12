use std::path::PathBuf;

use clap::Parser;

use crate::handlers::config::CompleteConfig;

#[derive(Parser, Debug)]
#[clap(rename_all = "kebab-case")]
#[clap(author, version, about)]
/// Mass merge templates into repositories
pub struct Cli {
    /// Where the template repository is located locally.
    #[arg()]
    template: Option<PathBuf>,
    /// To what local repositories the template should be applied to.
    #[arg()]
    apply_to: Option<Vec<PathBuf>>,
    /// Which files/folders to ignore.
    #[arg(short, long)]
    ignore: Option<Vec<PathBuf>>,
}

pub fn merge_args_into_config(config: &mut CompleteConfig, args: Cli) {
    if let Some(ignore) = args.ignore {
        config.ignore = Some(ignore);
    }
}
