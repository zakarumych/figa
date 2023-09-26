use core::fmt;

#[cfg(any(feature = "std", feature = "hashbrown"))]
use core::hash::{BuildHasher, Hash};

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

use serde::{
    de::{DeserializeOwned, DeserializeSeed, Visitor},
    Deserializer,
};

pub struct Append<'a, F>(pub &'a mut F);

impl<'de, T> DeserializeSeed<'de> for Append<'_, Option<T>>
where
    T: DeserializeOwned,
{
    type Value = ();

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(self)
    }
}

impl<'de, T> Visitor<'de> for Append<'_, Option<T>>
where
    T: DeserializeOwned,
{
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("option")
    }

    #[inline]
    fn visit_none<E>(self) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        Ok(())
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        *self.0 = Some(T::deserialize(deserializer)?);
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl<'de> DeserializeSeed<'de> for Append<'_, String> {
    type Value = ();

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(self)
    }
}

#[cfg(feature = "alloc")]
impl<'de> Visitor<'de> for Append<'_, String> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.0.push_str(value);
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl<'de, F> DeserializeSeed<'de> for Append<'_, Vec<F>>
where
    F: DeserializeOwned,
{
    type Value = ();

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(self)
    }
}

#[cfg(feature = "alloc")]
impl<'de, F> Visitor<'de> for Append<'_, Vec<F>>
where
    F: DeserializeOwned,
{
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("sequence")
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<(), A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        while let Some(value) = seq.next_element()? {
            self.0.push(value);
        }
        Ok(())
    }
}

#[cfg(any(feature = "alloc", feature = "hashbrown"))]
macro_rules! append_map {
    ($($map:ident)::+ <K, V $(, $tail:ident)*> $(where $($param:ident : $bound:path),* $(,)?)?) => {
        impl<'de, K, V $(, $tail)*> DeserializeSeed<'de> for Append<'_, $($map)::+ <K, V $(, $tail)*>>
        where
            K: DeserializeOwned,
            V: DeserializeOwned,
            $($($param: $bound,)*)?
        {
            type Value = ();

            #[inline]
            fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_map(self)
            }
        }

        impl<'de, K, V $(, $tail)*> Visitor<'de> for Append<'_, $($map)::+ <K, V $(, $tail)*>>
        where
            K: DeserializeOwned,
            V: DeserializeOwned,
            $($($param: $bound,)*)?
        {
            type Value = ();

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("map")
            }

            #[inline]
            fn visit_map<X>(self, mut map: X) -> Result<(), X::Error>
            where
                X: serde::de::MapAccess<'de>,
            {
                while let Some((key, value)) = map.next_entry()? {
                    self.0.insert(key, value);
                }
                Ok(())
            }
        }
    };
}

#[cfg(any(feature = "alloc", feature = "hashbrown"))]
macro_rules! append_set {
    ($($set:ident)::+ <T $(, $tail:ident)*> $(where $($param:ident : $bound:path),* $(,)?)?) => {
        impl<'de, T $(, $tail)*> DeserializeSeed<'de> for Append<'_, $($set)::+ <T $(, $tail)*>>
        where
            T: DeserializeOwned,
            $($($param: $bound,)*)?
        {
            type Value = ();

            #[inline]
            fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_seq(self)
            }
        }

        impl<'de, T $(, $tail)*> Visitor<'de> for Append<'_, $($set)::+ <T $(, $tail)*>>
        where
            T: DeserializeOwned,
            $($($param: $bound,)*)?
        {
            type Value = ();

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("set")
            }

            #[inline]
            fn visit_seq<X>(self, mut seq: X) -> Result<(), X::Error>
            where
                X: serde::de::SeqAccess<'de>,
            {
                while let Some(value) = seq.next_element()? {
                    self.0.insert(value);
                }
                Ok(())
            }
        }
    };
}

#[cfg(feature = "alloc")]
append_map!(alloc::collections::BTreeMap<K, V> where K: Ord);

#[cfg(feature = "alloc")]
append_set!(alloc::collections::BTreeSet<T> where T: Ord);

#[cfg(feature = "std")]
append_map!(std::collections::HashMap<K, V, S> where K: Eq, K: Hash, S: BuildHasher);

#[cfg(feature = "std")]
append_set!(std::collections::HashSet<T, S> where T: Eq, T: Hash, S: BuildHasher);

#[cfg(feature = "hashbrown")]
append_map!(hashbrown::HashMap<K, V, S> where K: Eq, K: Hash, S: BuildHasher);

#[cfg(feature = "hashbrown")]
append_set!(hashbrown::HashSet<T, S> where T: Eq, T: Hash, S: BuildHasher);
