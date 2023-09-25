// This is hack module!
//
// This `io` crate is bundled as an `io` module. The bundler basically translates `crate::sub_item`
// to `crate::io::sub_item` accordingly, but it is not the case with macros. Since `syn` parser does
// not recognize what is inside macro definition (just as a TokenStream), we can't translate those
// paths to correct one.
//
// We workaround this by preparing submodule named `io` and re-exporting everything so that we can
// access items in two ways, `io::something` and `io::io::something`.
//
// Consider `$crate::io::something` in macro definition (hence not translated by bundler)...
//
// - When you are developing locally, `$crate` is this `io` crate itself, so the path resolves to
//   `io::io::something`.
// - When bundled, `$crate` is the root crate, and `io` crate is translated to a submodule of that
//   root crate. Then `$crate::io::something` is what is originally called `io::something`.
//
// Introducing this re-exports, `io::something` and `io::io::something` points to the same item, so
// they compiles in both environment.
pub use super::*;
