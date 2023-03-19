use lazy_static::lazy_static;
use proconio::{input, source::auto::AutoSource};
use std::{
    env::args,
    fmt,
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::Path,
    sync::Mutex,
};

lazy_static! {
    static ref SOURCE: Mutex<AutoSource<Box<dyn BufRead + Send>>> = Mutex::new(find_source());
}

fn find_source() -> AutoSource<Box<dyn BufRead + Send>> {
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

#[derive(Debug, Clone)]
pub enum Output {}

impl fmt::Display for Output {
    fn fmt(&self, _b: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InitInput {}

impl InitInput {
    pub fn read() -> InitInput {
        let mut source = SOURCE
            .lock()
            .expect("internal error: failed to lock input source");
        input! {
            from &mut *source,
        }

        InitInput {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TurnInput {}

impl TurnInput {
    pub fn read() -> TurnInput {
        let mut source = SOURCE
            .lock()
            .expect("internal error: failed to lock input source");
        input! {
            from &mut *source,
        }

        TurnInput {}
    }
}
