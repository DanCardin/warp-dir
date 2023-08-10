use etcetera::base_strategy::{BaseStrategy, Xdg};
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::path::PathBuf;

use crate::shell::expand_path;

/// This currently needs to load eagerly in a static in order to satisfy the type
/// requirements of clap's PossibleValue struct.
pub static TARGETS_FILE: Lazy<TargetsFile> = Lazy::new(|| {
    let config_path = if let Some(config_path) = std::env::var_os("WD_CONFIG") {
        Path::new(&config_path).to_path_buf()
    } else {
        let strategy = Xdg::new().unwrap();
        strategy.data_dir().join("wd").join("config")
    };

    match TargetsFile::read(&config_path) {
        Ok(targets_file) => targets_file,
        Err(err) => {
            eprintln!("Failed to load config file: {err}");
            std::process::exit(1);
        }
    }
});

#[derive(Clone)]
pub struct TargetsFile {
    config_path: PathBuf,
    pub targets: HashMap<String, PathBuf>,
}

impl TargetsFile {
    pub fn add(&mut self, name: Option<&str>, path: Option<&Path>) -> anyhow::Result<()> {
        let name = get_name(name)?;
        let path = get_path(path)?;

        if let Some(existing_path) = self.targets.get(&name) {
            if existing_path != &path {
                anyhow::bail!(
                    "Target {name} already exists, with a different path: {path}.",
                    path = path.to_string_lossy()
                );
            } else {
                anyhow::bail!("Target {name} already exists.");
            }
        }

        self.targets.insert(name, path);
        Ok(())
    }

    pub fn remove(&mut self, name: Option<&str>) -> anyhow::Result<()> {
        let name = get_name(name)?;

        if self.targets.contains_key(&name) {
            self.targets.remove(&name);
        } else {
            anyhow::bail!("Target {name} does not exist.");
        }
        Ok(())
    }

    pub fn write(&self) -> anyhow::Result<()> {
        let mut file = File::options()
            .write(true)
            .truncate(true)
            .open(&self.config_path)?;

        let content = self
            .items()
            .map(|(name, path)| [name.as_ref(), path.to_string_lossy().as_ref()].join(":"))
            .join("\n");

        file.write_all(content.as_ref())?;
        Ok(())
    }

    pub fn read(path: &Path) -> anyhow::Result<Self> {
        let content = if let Ok(file) = File::open(path) {
            let mut reader = BufReader::new(file);

            let mut contents = String::new();
            reader.read_to_string(&mut contents).unwrap_or(0);
            contents
        } else {
            String::new()
        };

        let targets = content
            .lines()
            .flat_map(|i| i.split_once(':'))
            .map(|(left, right)| -> anyhow::Result<_> {
                let full_path = expand_path(Path::new(right))?;
                Ok((left.to_string(), full_path))
            })
            .try_collect()?;

        Ok(Self {
            config_path: path.to_path_buf(),
            targets,
        })
    }

    pub fn get(&self, name: &str) -> Option<&PathBuf> {
        self.targets.get(name)
    }

    pub fn items(&self) -> impl Iterator<Item = (&String, &PathBuf)> {
        self.targets.iter().sorted()
    }
}

pub fn get_name(name: Option<&str>) -> anyhow::Result<String> {
    let name = if let Some(n) = name {
        n.to_string()
    } else {
        current_dir()?
            .file_name()
            .ok_or(anyhow::anyhow!("Current directory has no name"))?
            .to_string_lossy()
            .to_string()
    };
    Ok(name)
}

fn get_path(path: Option<&Path>) -> anyhow::Result<PathBuf> {
    let path = if let Some(p) = path {
        p.to_path_buf()
    } else {
        current_dir()?
    };
    Ok(path)
}
