//! # Figa is layered configuration library for Rust.
//!
//! Figa provides a way to load configuration values from multiple sources and update them into a single value.
//! How values are updated is controlled by the type of the value.
//! Figa uses `serde` for deserialization.
//! Derive `Figa` trait for structures to make them updateable.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "load")]
pub mod load;

mod append;
mod default;
mod replace;
mod update;

use serde::Deserializer;

#[cfg(feature = "derive")]
pub use figa_proc::Figa;

/// Trait for loadable layered configuration values.
/// Uses `serde` for deserialization.
/// Can be updated with other values of the same type to make a layered configuration.
///
/// Can be derived for user-defined types.
/// Derive macro accepts attributes to control how fields are loaded.
/// - `#[figa(update)]` causes the field to be updated with the value from the next layer.
///   This is default behavior when no attributes are specified.
///   Field type must implement `Figa` trait.
/// - `#[figa(replace)]` causes the field to be replaced with the value from the next layer. Field must implement `serde::Deserialize`.
/// - `#[figa(append)]` causes the field to be appended with the value from the next layer.
///   Works on collections like `Vec` and `HashSet`, `HashMap`. Values with equal keys are replaced.
///   Value type must implement `serde::Deserialize`.
/// - `#[figa(union)]` similar to the above but works for `HashSet`, `HashMap` and similar collections.
///   Values with equal keys are updated. Value type must implement `Figa` trait.
///   Value type must implement `serde::Deserialize` and `Figa`.
///
/// This trait only defines `load` method to load next layer to the existing configuration value.
/// First should be loaded by other means.
/// Some functions load the first layer using `serde::Deserialize` trait.
pub trait Figa {
    /// Update next layer from a deserializer.
    fn update<'de, D>(&mut self, deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>;
}

/// This module is used by the derive macro.
/// It is not intended to be used directly.
/// Its content is not under semantic versioning.
#[doc(hidden)]
pub mod private {
    pub use str;

    pub use core::{
        fmt::{Formatter, Result as FmtResult},
        hash::Hash,
        result::Result::{self, Err, Ok},
    };

    pub use serde::de::{
        Deserialize, DeserializeOwned, DeserializeSeed, Deserializer, Error as DeError, MapAccess,
        SeqAccess, Visitor,
    };

    pub use crate::{append::Append, default::Default, replace::Replace, update::Update, Figa};

    pub struct UnitStructVisitor;

    impl<'de> Visitor<'de> for UnitStructVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            formatter.write_str("unit struct")
        }

        fn visit_unit<E>(self) -> Result<(), E>
        where
            E: DeError,
        {
            Ok(())
        }
    }
}
