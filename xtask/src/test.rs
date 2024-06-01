use anyhow::Result;

pub mod ahc;

pub fn main(args: &[String]) -> Result<()> {
    ahc::main()
}
