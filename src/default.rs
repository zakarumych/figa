#[cfg(any(feature = "std", feature = "hashbrown"))]
use core::hash::{BuildHasher, Hash};

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

use serde::{
    de::{DeserializeOwned, DeserializeSeed},
    Deserializer,
};

use crate::Figa;

/// Uses default update behavior for the type.
/// Not all types have default behavior, but usually type that supports
/// at least one update behavior has default behavior.
pub struct Default<'a, F>(pub &'a mut F);

macro_rules! default_replace {
        ($($types:ty $(where $($params:ident : $head_bound:ident $(+ $tail_bounds:ident)*),+)?)*) => {
            $(
                impl<'de $($(, $params : $head_bound $(+ $tail_bounds)*)+)?> DeserializeSeed<'de> for Default<'_, $types> {
                    type Value = ();

                    #[inline]
                    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        serde::Deserialize::deserialize_in_place(deserializer, self.0)
                    }
                }
            )*
        };
    }

#[cfg(any(feature = "alloc", feature = "hashbrown"))]
macro_rules! default_append {
        ($($types:ty $(where $($params:ident : $head_bound:ident $(+ $tail_bounds:ident)*),+)?);* $(;)?) => {
            $(
                impl<'de $($(, $params : $head_bound $(+ $tail_bounds)*)+)?> DeserializeSeed<'de> for Default<'_, $types> {
                    type Value = ();

                    #[inline]
                    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
                    where
                        D: serde::de::Deserializer<'de>,
                    {
                        crate::append::Append(self.0).deserialize(deserializer)
                    }
                }
            )*
        };
    }

macro_rules! default_update {
        ($($types:ty $(where $($params:ident : $head_bound:ident $(+ $tail_bounds:ident)*),+)?);* $(;)?) => {
            $(
                impl<'de $($(, $params : $head_bound $(+ $tail_bounds)*)+)?> DeserializeSeed<'de> for Default<'_, $types> {
                    type Value = ();

                    #[inline]
                    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        crate::update::Update(self.0).deserialize(deserializer)
                    }
                }
            )*
        };
    }

default_replace!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 bool char);

#[cfg(feature = "alloc")]
default_replace!(String);

#[cfg(feature = "alloc")]
default_append!(Vec<T> where T: DeserializeOwned);

#[cfg(feature = "alloc")]
default_append! {
    alloc::collections::BTreeMap<K, V> where
        K: DeserializeOwned + Ord,
        V: DeserializeOwned;

    alloc::collections::BTreeSet<T> where
        T: DeserializeOwned + Ord;
}

#[cfg(feature = "std")]
default_append! {
    std::collections::HashMap<K, V, S> where
        K: DeserializeOwned + Eq + Hash,
        V: DeserializeOwned,
        S: BuildHasher;

    std::collections::HashSet<T, S> where
        T: DeserializeOwned + Eq + Hash,
        S: BuildHasher;
}

#[cfg(feature = "hashbrown")]
default_append! {
    hashbrown::HashMap<K, V, S> where
        K: DeserializeOwned + Eq + Hash,
        V: DeserializeOwned,
        S: BuildHasher;

    hashbrown::HashSet<T, S> where
        T: DeserializeOwned + Eq + Hash,
        S: BuildHasher;
}

default_update! {
    T where T: Figa;
    Option<T> where T: Figa + DeserializeOwned;
}
