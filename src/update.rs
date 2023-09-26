use core::fmt;

#[cfg(any(feature = "std", feature = "hashbrown"))]
use core::hash::{BuildHasher, Hash};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use serde::{
    de::{DeserializeOwned, DeserializeSeed, Visitor},
    Deserializer,
};

use crate::Figa;

pub struct Update<'a, T>(pub &'a mut T);

impl<'de, T> DeserializeSeed<'de> for Update<'_, T>
where
    T: Figa,
{
    type Value = ();

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        self.0.update(deserializer)
    }
}

impl<'de, T> DeserializeSeed<'de> for Update<'_, Option<T>>
where
    T: DeserializeOwned + Figa,
{
    type Value = ();

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateVisitor<'a, T>(&'a mut Option<T>);

        impl<'de, T> Visitor<'de> for UpdateVisitor<'_, T>
        where
            T: DeserializeOwned + Figa,
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
                match self.0 {
                    None => *self.0 = Some(T::deserialize(deserializer)?),
                    Some(value) => value.update(deserializer)?,
                }
                Ok(())
            }
        }

        deserializer.deserialize_option(UpdateVisitor(self.0))
    }
}

#[cfg(feature = "alloc")]
impl<'de, T> DeserializeSeed<'de> for Update<'_, Vec<T>>
where
    T: DeserializeOwned + Figa,
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
impl<'de, T> Visitor<'de> for Update<'_, Vec<T>>
where
    T: DeserializeOwned + Figa,
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
        let mut idx = 0;
        loop {
            if idx >= self.0.len() {
                let Some(value) = seq.next_element()? else {
                    break;
                };
                self.0.push(value);
            } else {
                if seq.next_element_seed(Update(&mut self.0[idx]))?.is_none() {
                    break;
                }
            }
            idx += 1;
        }
        Ok(())
    }
}

#[cfg(any(feature = "alloc", feature = "hashbrown"))]
macro_rules! update_map {
    ($($q:ident::)*{Entry, $map:ident<K, V $(, $tail:ident)*>} $(where $($param:ident : $bound:path),* $(,)?)?) => {
        impl<'a, 'de, K, V $(, $tail)*> DeserializeSeed<'de> for Update<'a, $($q::)* $map<K, V $(, $tail)*>>
        where
            K: DeserializeOwned,
            V: DeserializeOwned + Figa,
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


        impl<'a, 'de, K, V $(, $tail)*> Visitor<'de> for Update<'a, $($q::)* $map<K, V $(, $tail)*>>
        where
            K: DeserializeOwned,
            V: DeserializeOwned + Figa,
            $($($param: $bound,)*)?
        {
            type Value = ();

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("map")
            }

            #[inline]
            fn visit_map<M>(self, mut map: M) -> Result<(), M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                while let Some(key) = map.next_key()? {
                    match self.0.entry(key) {
                        $($q::)* Entry::Occupied(mut entry) => {
                            map.next_value_seed(Update(entry.get_mut()))?;
                        }
                        $($q::)* Entry::Vacant(entry) => {
                            entry.insert(map.next_value()?);
                        }
                    }
                }
                Ok(())
            }
        }
    };
}

#[cfg(feature = "alloc")]
update_map!(alloc::collections::btree_map::{Entry, BTreeMap<K, V>} where K: Ord);

#[cfg(feature = "std")]
update_map!(std::collections::hash_map::{Entry, HashMap<K, V, S>} where K: Eq, K: Hash, S: BuildHasher);

#[cfg(feature = "hashbrown")]
update_map!(hashbrown::hash_map::{Entry, HashMap<K, V, S>} where K: Eq, K: Hash, S: BuildHasher);
