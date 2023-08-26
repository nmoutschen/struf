#![warn(missing_docs, clippy::missing_docs_in_private_items)]
//! # Struf: Struct Filters

pub use struf_derive::Filter;

/// Create a filter for this struct
///
/// ## Usage
///
/// ```rust
/// use struf::Filter;
///
/// #[derive(Filter)]
/// pub struct MyStruct {
///     #[filter]
///     pub name: String,
/// }
///
/// let mut filter = MyStruct::filter().with_name("my_name".to_string());
/// ```
pub trait Filter {
    /// Specific filter corresponding to this struct
    type Filter;

    /// Create a new [`Filter`] for this type with no rules
    fn filter() -> Self::Filter;
}
