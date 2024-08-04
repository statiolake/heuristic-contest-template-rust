use anyhow::{bail, ensure, Result};
use std::{
    env::{self, args},
    path::PathBuf,
};

pub mod bundle;
pub mod test;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    ensure!(args.len() > 1, "no task specified");
    ensure_project_root()?;

    match &*args[1] {
        "bundle" => bundle::main(&args[2..]),
        "test" => test::main(&args[2..]),
        _ => bail!("unknown task: {}", args[1]),
    }
}

fn ensure_project_root() -> Result<()> {
    let manifest_path = find_upwards("Cargo.toml")?;
    let project_root = manifest_path.parent().expect("Cargo.toml has no parent");
    env::set_current_dir(project_root)?;

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
