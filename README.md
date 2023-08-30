# **Struf**: Struct Filters

Automatically create filters for `struct`s.

**Warning**: this crate is highly experimental. Use at your own risk, and please report any bugs on
as issues on GitHub.

## Usage

```rust
use struf::Filter;

#[derive(Filter)]
pub struct MyStruct {
    #[filter]
    pub name: String,
}

// Filter on a single value
let filter = MyStruct::filter().with_name("my_name");

// Filter on multiple values
let filter = MyStruct::filter().with_names(vec!["name_a", "name_b"]);

// Access filter values
dbg!(&filter.names);

// Use it to match against existing values
filter.matches(&MyStruct {
    name: "Some name".to_string(),
});
```