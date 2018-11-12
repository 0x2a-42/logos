#![warn(missing_docs)]

#[cfg(feature = "nul_term_source")]
extern crate toolshed;

mod lexer;
mod source;

#[doc(hidden)]
pub mod internal;

use std::ops::Range;
use internal::LexerInternal;

pub use lexer::{Lexer, Lexicon, Extras};
pub use source::Source;

/// Trait implemented for an enum representing all tokens. You should never have
/// to implement it manually, use the `#[derive(Logos)]` attribute on your enum.
pub trait Logos: Sized {
    /// Associated `Extras` for the particular lexer. Those can handle things that
    /// aren't necessarily tokens, such as comments or Automatic Semicolon Insertion
    /// in JavaScript.
    type Extras: self::Extras;

    /// `SIZE` is simply a number of possible variants of the `Logos` enum. The
    /// `derive` macro will make sure that all variants don't hold values larger
    /// or equal to `SIZE`.
    ///
    /// This can be extremely useful for creating `Logos` Lookup Tables.
    const SIZE: usize;

    /// Helper const pointing to the variant marked as #[error].
    const ERROR: Self;

    /// Returns a lookup table for the `Lexer`
    fn lexicon<S: Source>() -> Lexicon<Lexer<Self, S>>;

    /// Create a new instance of a `Lexer` that will produce tokens implementing
    /// this `Logos`.
    fn lexer<S: Source>(source: S) -> Lexer<Self, S> {
        Lexer::new(source)
    }
}
