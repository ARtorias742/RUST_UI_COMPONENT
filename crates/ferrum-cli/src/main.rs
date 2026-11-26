use std::error::Error;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use ferrum_ui::registry::{add_component, diff_lines, Registry, RegistryError};

#[derive(Debug, Parser)]
#[command(name = "ferrum", version, about = "Source distribution for Ferrum UI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Copy a component template into the destination crate.
    Add {
        name: String,
        #[arg(default_value = ".")]
        dest: PathBuf,
        #[arg(long)]
        force: bool,
    },
    /// List templates in the registry.
    List,
    /// Diff an on-disk file against its template.
    Diff { name: String, path: PathBuf },
    /// Update a template to a newer semantic version.
    Update { name: String, version: String },
}

fn registry_path() -> PathBuf {
    PathBuf::from("ferrum.registry.json")
}

fn load_registry() -> Result<Registry, RegistryError> {
    let path = registry_path();
    if path.exists() {
        Registry::load(&path)
    } else {
        Ok(Registry::builtin())
    }
}

fn find_file<'a>(
    registry: &'a Registry,
    name: &str,
    path: &Path,
) -> Result<&'a str, RegistryError> {
    let manifest = registry
        .get(name)
        .ok_or_else(|| RegistryError::NotFound(name.to_owned()))?;
    let target = path.to_string_lossy();
    let file = manifest
        .files
        .iter()
        .find(|file| {
            file.path == target
                || Path::new(&file.path) == path
                || Path::new(&file.path).file_name() == path.file_name()
        })
        .ok_or_else(|| RegistryError::NotFound(target.into_owned()))?;
    Ok(&file.contents)
}

fn run() -> Result<(), Box<dyn Error>> {
    match Cli::parse().command {
        Command::Add { name, dest, force } => {
            let registry = load_registry()?;
            if registry
                .get(&name)
                .is_some_and(|manifest| manifest.deprecated)
            {
                eprintln!("ferrum: warning: `{name}` is deprecated");
            }
            let report = add_component(&registry, &name, &dest, force)?;
            for path in report.written {
                println!("wrote {}", path.display());
            }
            for path in report.skipped {
                println!("skipped {} (use --force)", path.display());
            }
        }
        Command::List => {
            for manifest in load_registry()?.list() {
                if manifest.deprecated {
                    println!("{} {} (deprecated)", manifest.name, manifest.version);
                } else {
                    println!("{} {}", manifest.name, manifest.version);
                }
            }
        }
        Command::Diff { name, path } => {
            let registry = load_registry()?;
            let old = find_file(&registry, &name, &path)?;
            let new = std::fs::read_to_string(&path)?;
            print!("{}", diff_lines(old, &new));
        }
        Command::Update { name, version } => {
            let mut registry = load_registry()?;
            let files = registry
                .get(&name)
                .ok_or_else(|| RegistryError::NotFound(name.clone()))?
                .files
                .clone();
            registry.update(&name, &version, files)?;
            registry.save(&registry_path())?;
            println!("updated {name} to {version}");
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("ferrum: {error}");
        std::process::exit(1);
    }
}
