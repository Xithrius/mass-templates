use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{bail, Error, Result};
use serde::{Deserialize, Serialize};

use crate::{
    handlers::args::{merge_args_into_config, Cli},
    utils::pathing::config_path,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct CompleteConfig {
    /// Where the template repository is located locally.
    pub template: Option<PathBuf>,
    /// To what local repositories the template should be applied to.
    pub apply_to: Option<Vec<PathBuf>>,
    /// Which files/folders to ignore.
    pub ignore: Option<Vec<PathBuf>>,
}

impl CompleteConfig {
    pub fn new(cli: Cli) -> Result<Self, Error> {
        let path_str = config_path("config.toml");

        let p = Path::new(&path_str);

        if !p.exists() {
            create_dir_all(p.parent().unwrap()).unwrap();

            let default_toml_string = toml::to_string(&Self::default()).unwrap();
            let mut file = File::create(path_str.clone()).unwrap();
            file.write_all(default_toml_string.as_bytes()).unwrap();

            bail!("Configuration was generated at {path_str}, please fill it out with necessary information.")
        } else if let Ok(config_contents) = read_to_string(p) {
            let mut config: Self = toml::from_str(config_contents.as_str()).unwrap();

            merge_args_into_config(&mut config, cli);

            Ok(config)
        } else {
            bail!(
                "Configuration could not be read correctly. See the following link for the example config: {}",
                format!("{}/blob/main/default-config.toml", env!("CARGO_PKG_REPOSITORY"))
            )
        }
    }
}
