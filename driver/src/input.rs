use proconio::{input, source::auto::AutoSource};
use std::{
    env::args,
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Input {}

impl Input {
    pub fn read() -> Input {
        let source = find_source();

        input! {
            from source,
        }

        Input {}
    }
}

fn find_source() -> AutoSource<Box<dyn BufRead>> {
    if let Some(name) = args().nth(1) {
        let path = Path::new(&name);
        if path.exists() {
            eprintln!("input: {}", path.display());
            let f = File::open(path).expect("internal error: failed to open input file");
            let br = BufReader::new(f);
            return AutoSource::new(Box::new(br) as Box<_>);
        } else {
            eprintln!("file {} does not exist", path.display());
        }
    }

    eprintln!("input: stdin");
    AutoSource::new(Box::new(BufReader::new(stdin())) as Box<_>)
}
