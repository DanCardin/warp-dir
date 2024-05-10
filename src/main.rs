use clap::builder::PossibleValue;
use itertools::Itertools;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::{Arg, ArgAction, Args, Command, FromArgMatches, Parser, Subcommand};

use clap_complete::generate;

mod shell;
mod target;

use shell::Shell;

use self::target::{get_name, TargetsFile};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Optional path component. Warps to the directory specified by the warp point
    /// with path appended
    path: Option<PathBuf>,

    #[arg(
        short,
        long,
        action = clap::ArgAction::Set,
        value_parser = clap_complete::Shell::from_str,
    )]
    shell: Option<clap_complete::Shell>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Adds the current working directory to your warp points
    Add(AddCommand),

    /// Removes the given warp point
    Remove(RemoveCommand),

    /// Print all stored warp points
    List,

    /// Show the path to given warp point (pwd)
    Show(ShowCommand),

    /// Remove points warping to nonexistent directories
    Clean,

    /// Subcommands for hooking into the shell
    Shell(ShellCommand),
}

#[derive(Parser, Debug)]
struct AddCommand {
    name: Option<String>,

    #[arg(short, long)]
    path: Option<PathBuf>,
}

#[derive(Parser, Debug)]
struct RemoveCommand {
    name: Option<String>,
}

#[derive(Parser, Debug)]
struct ShowCommand {
    name: Option<String>,
}

#[derive(Parser, Debug)]
pub struct ShellCommand {
    #[command(subcommand)]
    command: ShellCommands,
}

#[derive(Subcommand, Debug)]
enum ShellCommands {
    Init,
    Completion,
}

fn main() -> anyhow::Result<()> {
    let cli_name = "wd";

    let targets_file = &target::TARGETS_FILE;
    let pv: Vec<PossibleValue> = targets_file
        .items()
        .map(|(pv, help)| PossibleValue::new(pv).help(help.to_str()))
        .collect();

    let cli = Command::new(cli_name).arg(
        Arg::new("target")
            .action(ArgAction::Set)
            .value_parser(pv)
            .help("Warps to the directory specified by the warp point"),
    );
    let mut cli = Cli::augment_args(cli);

    let matches = cli.clone().get_matches();
    let args = Cli::from_arg_matches(&matches)
        .map_err(|err| err.exit())
        .unwrap();

    let clap_shell = args
        .shell
        .unwrap_or(clap_complete::Shell::from_env().unwrap_or(clap_complete::Shell::Bash));

    let shell_name = format!("{}", clap_shell);
    let shell = Shell::new(&shell_name);

    match args.command {
        Some(Commands::Shell(cmd)) => match cmd.command {
            ShellCommands::Completion => {
                generate(clap_shell, &mut cli, cli_name, &mut std::io::stdout())
            }
            ShellCommands::Init => shell.init()?,
        },
        Some(Commands::Add(cmd)) => {
            let mut target_file = targets_file.deref().clone();
            target_file.add(cmd.name.as_deref(), cmd.path.as_deref())?;
            target_file.write()?;
        }
        Some(Commands::Remove(cmd)) => {
            let mut target_file = targets_file.deref().clone();
            target_file.remove(cmd.name.as_deref())?;
            target_file.write()?;
        }
        Some(Commands::List) => {
            let list = targets_file
                .items()
                .map(|(name, path)| {
                    format!(
                        " - {name} -> {path}",
                        name = name,
                        path = path.to_string_lossy()
                    )
                })
                .join("\n");

            eprintln!("{list}");
        }
        Some(Commands::Show(cmd)) => {
            let name = get_name(cmd.name.as_deref())?;
            if let Some(path) = targets_file.get(&name) {
                eprintln!("{}", path.to_string_lossy());
            } else {
                eprintln!("Found no warp target: {name}");
            }
        }
        Some(Commands::Clean) => {
            let removed_targets = clean(targets_file.deref())?;

            if removed_targets.is_empty() {
                eprintln!("No targets to remove");
            } else {
                let result = removed_targets
                    .iter()
                    .map(|(name, path)| {
                        format!(
                            " - {name} -> {path}",
                            name = name,
                            path = path.to_string_lossy()
                        )
                    })
                    .join("\n");

                eprintln!("Removed:\n{result}");
            }
        }
        None => {
            if let Some(name) = matches.get_one::<String>("target") {
                let path = targets_file.get(name).unwrap();
                shell.invoke(path);
            } else {
                shell.invoke(Path::new("~"));
                todo!()
            }
        }
    };
    Ok(())
}

fn clean(target_file: &TargetsFile) -> anyhow::Result<Vec<(String, PathBuf)>> {
    let mut removable_targets = vec![];

    for (name, path) in target_file.items() {
        if !path.exists() {
            removable_targets.push((name.clone(), path.clone()));
        }
    }

    let mut target_file = target_file.clone();
    for (name, _) in removable_targets.iter() {
        target_file.remove(Some(name))?;
    }
    target_file.write()?;
    Ok(removable_targets)
}
