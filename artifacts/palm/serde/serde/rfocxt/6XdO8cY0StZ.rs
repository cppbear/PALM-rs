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
pub struct UnitOnly<E> {
    marker: PhantomData<E>,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Unexpected<'a> {
    /// The input contained a boolean value that was not expected.
    Bool(bool),
    /// The input contained an unsigned integer `u8`, `u16`, `u32` or `u64` that
    /// was not expected.
    Unsigned(u64),
    /// The input contained a signed integer `i8`, `i16`, `i32` or `i64` that
    /// was not expected.
    Signed(i64),
    /// The input contained a floating point `f32` or `f64` that was not
    /// expected.
    Float(f64),
    /// The input contained a `char` that was not expected.
    Char(char),
    /// The input contained a `&str` or `String` that was not expected.
    Str(&'a str),
    /// The input contained a `&[u8]` or `Vec<u8>` that was not expected.
    Bytes(&'a [u8]),
    /// The input contained a unit `()` that was not expected.
    Unit,
    /// The input contained an `Option<T>` that was not expected.
    Option,
    /// The input contained a newtype struct that was not expected.
    NewtypeStruct,
    /// The input contained a sequence that was not expected.
    Seq,
    /// The input contained a map that was not expected.
    Map,
    /// The input contained an enum that was not expected.
    Enum,
    /// The input contained a unit variant that was not expected.
    UnitVariant,
    /// The input contained a newtype variant that was not expected.
    NewtypeVariant,
    /// The input contained a tuple variant that was not expected.
    TupleVariant,
    /// The input contained a struct variant that was not expected.
    StructVariant,
    /// A message stating what uncategorized thing the input contained that was
    /// not expected.
    ///
    /// The message should be a noun or noun phrase, not capitalized and without
    /// a period. An example message is "unoriginal superhero".
    Other(&'a str),
}
impl<'de, E> de::VariantAccess<'de> for UnitOnly<E>
where
    E: de::Error,
{
    type Error = E;
    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(de::Error::invalid_type(Unexpected::UnitVariant, &"newtype variant"))
    }
    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::invalid_type(Unexpected::UnitVariant, &"tuple variant"))
    }
    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::invalid_type(Unexpected::UnitVariant, &"struct variant"))
    }
}
