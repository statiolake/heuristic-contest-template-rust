use anyhow::{bail, ensure, Result};
use std::env::args;

pub mod bundle;
pub mod test;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    ensure!(args.len() > 1, "no task specified");

    match &*args[1] {
        "bundle" => bundle::main(&args[2..]),
        "test" => test::main(&args[2..]),
        _ => bail!("unknown task: {}", args[1]),
    }
}
