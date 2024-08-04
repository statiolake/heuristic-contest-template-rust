use anyhow::{bail, ensure, Context as _, Result};
use std::{
    env::{self, args},
    error::Error,
    path::PathBuf,
    process,
};

pub mod bundle;
pub mod table;
pub mod test;

fn main() {
    if let Err(e) = imp() {
        eprintln!("error: {e}");
        let mut curr: &dyn Error = &*e;
        while let Some(cause) = curr.source() {
            eprintln!("caused by: {cause}");
            curr = cause;
        }

        process::exit(1);
    }
}

fn imp() -> Result<()> {
    let args: Vec<String> = args().collect();
    ensure!(args.len() > 1, "no task specified");
    ensure_project_root().context("failed to determine project root")?;

    match &*args[1] {
        "bundle" => bundle::main(&args[2..]).context("failed to bundle source"),
        "test" => test::main(&args[2..]).context("failed to run tests"),
        _ => bail!("unknown task: {}", args[1]),
    }
}

fn ensure_project_root() -> Result<()> {
    let manifest_path = find_upwards("Cargo.toml")?;
    let project_root = manifest_path.parent().expect("Cargo.toml has no parent");
    env::set_current_dir(project_root)?;
    eprintln!("project root: {}", project_root.display());

    Ok(())
}

fn find_upwards(name: &str) -> Result<PathBuf> {
    let mut current_dir = env::current_dir()?;
    loop {
        let path = current_dir.join(name);
        if path.is_file() {
            return Ok(path);
        }

        if !current_dir.pop() {
            bail!("could not find {}", name);
        }
    }
}
