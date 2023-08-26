#![warn(missing_docs, clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

pub use struf_derive::Filter;

/// Create a filter for this struct
pub trait Filter {
    /// Specific filter corresponding to this struct
    type Filter;

    /// Create a new [`Filter`] for this type with no rules
    fn filter() -> Self::Filter;
}
