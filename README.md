# Figa - layered configuration library.

Augments `serde` deserialization with customizeable updating mechanism.

## Usage

This crate provides `Figa` trait to update values using `serde` deserialization.
The trait uses single method `update`.\
The magic happens in derive macro. Enable `"derive"` feature to make it re-exported from `figa` crate, or use `figa-proc` crate directly.\
Derive macro currently supports only structs without generics.
User may customize update behavior for struct fields using `#[figa(*)]` attribute:
- `#[figa(replace)]` tells the codegen that field's value must be replaced with the new one.
- `#[figa(append)]` works for collections and tells the codegen that this field is a collection and new elements must be added.
  In case of maps, if value with the same key was present it will be replaced with new one.
  For `Vec` and `String` appended values are added to the end as expected.
- `#[figa(update)]` tells the codegen that field must be updated.
  This means that `Figa::update` will be called on for it recursively.
  For collections this means that values with same keys or indices are updated using `Figa` trait
  and new values are appended.

## Optional `load` feature

provides opinionated way to load configuration from `.toml` files in predefined set of locations.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
