use miette::Result;

pub mod ahc;

pub fn main(_args: &[String]) -> Result<()> {
    ahc::main()
}
