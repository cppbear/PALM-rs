pub type First<T> = <T as Pair>::First;
pub type Second<T> = <T as Pair>::Second;
use crate::lib::*;
use crate::de::{
    self, DeserializeSeed, Deserializer, MapAccess, Unexpected, VariantAccess, Visitor,
};
pub trait VariantAccess<'de>: Sized {
    type Error: Error;
    fn unit_variant(self) -> Result<(), Self::Error>;
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>;
    #[inline]
    fn newtype_variant<T>(self) -> Result<T, Self::Error>
    where
        T: Deserialize<'de>,
    {
        self.newtype_variant_seed(PhantomData)
    }
    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
}
pub struct MapAsEnum<A> {
    map: A,
}
#[derive(Debug)]
pub struct Error;
#[derive(Clone, PartialEq)]
pub struct Error {
    err: ErrorImpl,
}
impl<'de, A> VariantAccess<'de> for MapAsEnum<A>
where
    A: MapAccess<'de>,
{
    type Error = A::Error;
    fn unit_variant(mut self) -> Result<(), Self::Error> {
        self.map.next_value()
    }
    fn newtype_variant_seed<T>(mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        self.map.next_value_seed(seed)
    }
    fn tuple_variant<V>(
        mut self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.map.next_value_seed(SeedTupleVariant { len, visitor })
    }
    fn struct_variant<V>(
        mut self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.map.next_value_seed(SeedStructVariant { visitor })
    }
}
