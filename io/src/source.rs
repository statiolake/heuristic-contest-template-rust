use std::{
    any::type_name,
    fmt,
    io::{stdin, BufRead, BufReader, Stdin},
    iter::Peekable,
    ptr::NonNull,
    str::{FromStr, SplitWhitespace},
};

pub struct Source {
    tokens: Peekable<SplitWhitespace<'static>>,
    context: Option<NonNull<str>>,
    reader: BufReader<Stdin>,
}

impl Drop for Source {
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

impl Source {
    pub fn new() -> Self {
        let reader = BufReader::new(stdin());

        Self {
            tokens: "".split_whitespace().peekable(),
            context: None,
            reader,
        }
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

impl Default for Source {
    fn default() -> Self {
        Self::new()
    }
}

// SAFETY: `context` is not accessed directly.
unsafe impl Send for Source {}

pub trait Readable {
    type Output;
    fn read(source: &mut Source) -> Self::Output;
}

impl<T: FromStr> Readable for T
where
    T::Err: fmt::Debug,
{
    type Output = T;
    fn read(source: &mut Source) -> T {
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
    fn read(source: &mut Source) -> Vec<char> {
        source.next_token().unwrap().chars().collect()
    }
}

pub enum Bytes {}

impl Readable for Bytes {
    type Output = Vec<u8>;
    fn read(source: &mut Source) -> Vec<u8> {
        source.next_token().unwrap().bytes().collect()
    }
}

pub enum Usize1 {}

impl Readable for Usize1 {
    type Output = usize;
    fn read(source: &mut Source) -> usize {
        // panic if the subtraction overflows
        usize::read(source)
            .checked_sub(1)
            .expect("attempted to read the value 0 as a Usize1")
    }
}

pub enum Isize1 {}

impl Readable for Isize1 {
    type Output = isize;
    fn read(source: &mut Source) -> isize {
        isize::read(source)
            .checked_sub(1)
            .unwrap_or_else(|| panic!("attempted to read isize::MIN as Isize1"))
    }
}
