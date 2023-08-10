use path_absolutize::Absolutize;
use std::path::{Path, PathBuf};

use anyhow::Context;

pub struct Shell {
    name: String,
}

impl Shell {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn init(&self) -> anyhow::Result<()> {
        let exe_path = std::env::current_exe()?;
        let exe_name = exe_path
            .file_name()
            .context(format!("{exe_path:?} is has no file name"))?;

        let function = match self.name.as_ref() {
            "fish" => indoc::formatdoc!(
                r#"
                function {exe_name}
                  command {exe_path} --shell fish $argv | source
                end"#,
                exe_name = exe_name.to_string_lossy(),
                exe_path = exe_path.to_string_lossy(),
            ),
            // zsh/bash/etc. I.e. handle anything unknown with bash-like syntax
            _ => indoc::formatdoc!(
                r#"
                function {exe_name} {{
                  eval "$(command {exe_path} --shell {name} "$@")"
                }}"#,
                name = self.name,
                exe_name = exe_name.to_string_lossy(),
                exe_path = exe_path.to_string_lossy(),
            ),
        };

        println!("{}", function);

        Ok(())
    }

    pub fn invoke(&self, path: &Path) {
        println!("cd {}", path.to_string_lossy());
    }
}

pub fn expand_path<'a, P: Into<&'a Path>>(path: P) -> anyhow::Result<PathBuf> {
    Ok(shellexpand::path::tilde(path.into())
        .absolutize()?
        .to_path_buf())
}
