use std::{
    io::{BufRead, Write},
    sync::Mutex,
};

use crate::{source::Source, STDIN_SOURCE};

pub trait ReadInput {
    fn read_from<R: BufRead>(source: &mut Source<R>) -> Self
    where
        Self: Sized;

    fn read() -> Self
    where
        Self: Sized,
    {
        let mut locked_stdin = STDIN_SOURCE
            .get_or_init(|| Mutex::new(Source::new_stdin()))
            .lock()
            .unwrap();

        Self::read_from(&mut *locked_stdin)
    }
}

pub trait WriteOutput {
    fn write_to<W: Write>(&self, b: &mut W);

    fn write(&self) {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        self.write_to(&mut handle)
    }
}
