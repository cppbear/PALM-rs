pub type First<T> = <T as Pair>::First;
pub type Second<T> = <T as Pair>::Second;
use crate::lib::*;
use crate::de::{
    self, DeserializeSeed, Deserializer, MapAccess, Unexpected, VariantAccess, Visitor,
};
pub trait DeserializeSeed<'de>: Sized {
    type Value;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>;
}
struct SeedStructVariant<V> {
    visitor: V,
}
impl<'de, V> DeserializeSeed<'de> for SeedStructVariant<V>
where
    V: Visitor<'de>,
{
    type Value = V::Value;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(self.visitor)
    }
}
