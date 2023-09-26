use serde::{
    de::{DeserializeOwned, DeserializeSeed},
    Deserializer,
};

pub struct Replace<'a, F>(pub &'a mut F);

impl<'de, F> DeserializeSeed<'de> for Replace<'_, F>
where
    F: DeserializeOwned,
{
    type Value = ();

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        serde::Deserialize::deserialize_in_place(deserializer, self.0)
    }
}
