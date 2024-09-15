use std::sync::Mutex;

use crate::source::Source;

#[macro_export]
macro_rules! input {
    // terminator
    (@from [$source:expr] @rest) => {};

    // parse mutability
    (@from [$source:expr] @rest mut $($rest:tt)*) => {
        $crate::input! {
            @from [$source]
            @mut [mut]
            @rest $($rest)*
        }
    };
    (@from [$source:expr] @rest $($rest:tt)*) => {
        $crate::input! {
            @from [$source]
            @mut []
            @rest $($rest)*
        }
    };

    // parse variable pattern
    (@from [$source:expr] @mut [$($mut:tt)?] @rest $var:tt: $($rest:tt)*) => {
        $crate::input! {
            @from [$source]
            @mut [$($mut)*]
            @var $var
            @kind []
            @rest $($rest)*
        }
    };

    // parse kind (type)
    (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest) => {
        let $($mut)* $var = $crate::read_value!(@source [$source] @kind [$($kind)*]);
    };
    (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest, $($rest:tt)*) => {
        $crate::input!(@from [$source] @mut [$($mut)*] @var $var @kind [$($kind)*] @rest);
        $crate::input!(@from [$source] @rest $($rest)*);
    };
    (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest $tt:tt $($rest:tt)*) => {
        $crate::input!(@from [$source] @mut [$($mut)*] @var $var @kind [$($kind)* $tt] @rest $($rest)*);
    };

    (from $source:expr, $($rest:tt)*) => {
        #[allow(unused_variables, unused_mut)]
        let mut s = $source;
        $crate::input! {
            @from [&mut s]
            @rest $($rest)*
        }
    };
    ($($rest:tt)*) => {
        // This `io` is...
        // - A `io` submodule of `io` crate in local development; crate::io::io.
        // - Bundled `io` module (= original `io` crate root) after bundler.
        #[allow(unused_mut)]
        let mut locked_stdin = $crate::io::STDIN_SOURCE
            .get_or_init(|| std::sync::Mutex::new($crate::io::source::Source::new_stdin()))
            .lock()
            .unwrap();
        $crate::input! {
            @from [&mut *locked_stdin]
            @rest $($rest)*
        }
        drop(locked_stdin); // release the lock
    };
}

#[macro_export]
macro_rules! read_value {
    // array and variable length array
    (@source [$source:expr] @kind [[$($kind:tt)*]]) => {
        $crate::read_value!(@array @source [$source] @kind [] @rest $($kind)*)
    };
    (@array @source [$source:expr] @kind [$($kind:tt)*] @rest) => {{
        let len = <usize as $crate::io::source::Readable>::read($source);
        $crate::read_value!(@source [$source] @kind [[$($kind)*; len]])
    }};
    (@array @source [$source:expr] @kind [$($kind:tt)*] @rest ; $($rest:tt)*) => {
        $crate::read_value!(@array @source [$source] @kind [$($kind)*] @len [$($rest)*])
    };
    (@array @source [$source:expr] @kind [$($kind:tt)*] @rest $tt:tt $($rest:tt)*) => {
        $crate::read_value!(@array @source [$source] @kind [$($kind)* $tt] @rest $($rest)*)
    };
    (@array @source [$source:expr] @kind [$($kind:tt)*] @len [$($len:tt)*]) => {{
        let len = $($len)*;
        (0..len)
            .map(|_| $crate::read_value!(@source [$source] @kind [$($kind)*]))
            .collect::<Vec<_>>()
    }};

    // tuple
    (@source [$source:expr] @kind [($($kinds:tt)*)]) => {
        $crate::read_value!(@tuple @source [$source] @kinds [] @current [] @rest $($kinds)*)
    };
    (@tuple @source [$source:expr] @kinds [$([$($kind:tt)*])*] @current [] @rest) => {
        (
            $($crate::read_value!(@source [$source] @kind [$($kind)*]),)*
        )
    };
    (@tuple @source [$source:expr] @kinds [$($kinds:tt)*] @current [$($curr:tt)*] @rest) => {
        $crate::read_value!(@tuple @source [$source] @kinds [$($kinds)* [$($curr)*]] @current [] @rest)
    };
    (@tuple @source [$source:expr] @kinds [$($kinds:tt)*] @current [$($curr:tt)*] @rest, $($rest:tt)*) => {
        $crate::read_value!(@tuple @source [$source] @kinds [$($kinds)* [$($curr)*]] @current [] @rest $($rest)*)
    };
    (@tuple @source [$source:expr] @kinds [$($kinds:tt)*] @current [$($curr:tt)*] @rest $tt:tt $($rest:tt)*) => {
        $crate::read_value!(@tuple @source [$source] @kinds [$($kinds)*] @current [$($curr)* $tt] @rest $($rest)*)
    };

    // unreachable
    (@source [$source:expr] @kind []) => {
        compile_error!("Reached unreachable statement while parsing macro input");
    };

    // normal other
    (@source [$source:expr] @kind [$kind:ty]) => {
        <$kind as $crate::io::source::Readable>::read($source)
    };

    // human-friendly version
    ($($kind:tt)*) => {{
        // This `io` is...
        // - A `io` submodule of `io` crate in local development; crate::io::io.
        // - Bundled `io` module (= original `io` crate root) after bundler.
        #[allow(unused_mut)]
        let mut locked_stdin = $crate::io::STDIN_SOURCE
            .get_or_init(|| std::sync::Mutex::new($crate::io::source::Source::new_stdin()))
            .lock()
            .unwrap();
        $crate::read_value! {
            @source [&mut *locked_stdin]
            @kind [$($kind)*]
        }
    }};
}

pub fn is_stdin_empty() -> bool {
    let mut lock = crate::STDIN_SOURCE
        .get_or_init(|| Mutex::new(Source::new_stdin()))
        .lock()
        .expect("failed to lock stdin source");
    lock.is_empty()
}
