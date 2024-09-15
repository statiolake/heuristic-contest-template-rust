use std::{
    any::type_name,
    fmt,
    io::{stdin, BufRead, BufReader, Stdin},
    iter::Peekable,
    ptr::NonNull,
    str::{FromStr, SplitWhitespace},
};

pub struct Source<R = BufReader<Stdin>> {
    tokens: Peekable<SplitWhitespace<'static>>,
    context: Option<NonNull<str>>,
    reader: R,
}

impl<R> Drop for Source<R> {
    fn drop(&mut self) {
        // set dummy reference to `tokens`
        self.tokens = "".split_whitespace().peekable();

        // SAFETY: `tokens` no longer points to context
        if let Some(context) = self.context {
            unsafe {
                let _ = Box::from_raw(context.as_ptr());
                self.context = None;
            }
        }
    }
}

impl Source<BufReader<Stdin>> {
    pub fn new_stdin() -> Self {
        Self::new(BufReader::new(stdin()))
    }
}

impl<R: BufRead> Source<R> {
    pub fn new(reader: R) -> Self {
        Self {
            tokens: "".split_whitespace().peekable(),
            context: None,
            reader,
        }
    }

    pub fn as_mut(&mut self) -> Source<&mut dyn BufRead> {
        if self.context.is_some() {
            panic!("cannot borrow sources in use");
        }

        Source::new(&mut self.reader)
    }

    fn prepare(&mut self) {
        while self.tokens.peek().is_none() {
            let mut line = String::new();
            let num_bytes = self.reader.read_line(&mut line).expect("IO error");

            if num_bytes == 0 {
                // reached EOF
                return;
            }

            let context = NonNull::new(Box::leak(line.into_boxed_str())).unwrap();

            // SAFETY: we drop context only after dropping previous tokens (hence no reference to
            // context).
            let tokens = unsafe { (*context.as_ptr()).split_whitespace().peekable() };

            // Assign tokens first! This is important. After this assignment, there is no more references to previous context ...
            self.tokens = tokens;

            // ... so we can safely drop current context (if any).
            // SAFETY: tokens are already dropped.
            if let Some(context) = self.context {
                unsafe {
                    let _ = Box::from_raw(context.as_ptr());
                    self.context = Some(context);
                }
            }
        }
    }

    pub fn next_token(&mut self) -> Option<&str> {
        self.prepare();
        self.tokens.next()
    }

    pub fn is_empty(&mut self) -> bool {
        self.prepare();
        self.tokens.peek().is_none()
    }
}

impl Default for Source<BufReader<Stdin>> {
    fn default() -> Self {
        Self::new_stdin()
    }
}

// SAFETY: `context` is not accessed directly.
unsafe impl<R> Send for Source<R> {}

pub trait Readable {
    type Output;
    fn read<R: BufRead>(source: &mut Source<R>) -> Self::Output;
}

impl<T: FromStr> Readable for T
where
    T::Err: fmt::Debug,
{
    type Output = T;
    fn read<R: BufRead>(source: &mut Source<R>) -> T {
        let token = source.next_token().unwrap();
        match token.parse() {
            Ok(v) => v,
            Err(e) => panic!(
                "failed to parse the input `{}` to the value of type `{}`: {:?}",
                token,
                type_name::<T>(),
                e,
            ),
        }
    }
}

pub enum Chars {}

impl Readable for Chars {
    type Output = Vec<char>;
    fn read<R: BufRead>(source: &mut Source<R>) -> Vec<char> {
        source.next_token().unwrap().chars().collect()
    }
}

pub enum Bytes {}

impl Readable for Bytes {
    type Output = Vec<u8>;
    fn read<R: BufRead>(source: &mut Source<R>) -> Vec<u8> {
        source.next_token().unwrap().bytes().collect()
    }
}

pub enum Usize1 {}

impl Readable for Usize1 {
    type Output = usize;
    fn read<R: BufRead>(source: &mut Source<R>) -> usize {
        // panic if the subtraction overflows
        usize::read(source)
            .checked_sub(1)
            .expect("attempted to read the value 0 as a Usize1")
    }
}

pub enum Isize1 {}

impl Readable for Isize1 {
    type Output = isize;
    fn read<R: BufRead>(source: &mut Source<R>) -> isize {
        isize::read(source)
            .checked_sub(1)
            .unwrap_or_else(|| panic!("attempted to read isize::MIN as Isize1"))
    }
}
