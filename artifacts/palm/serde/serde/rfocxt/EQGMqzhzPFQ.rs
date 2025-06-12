#[cfg(any(feature = "std", feature = "alloc"))]
type ErrorImpl = Box<str>;
#[cfg(not(any(feature = "std", feature = "alloc")))]
type ErrorImpl = ();
use crate::lib::*;
use self::private::{First, Second};
use crate::de::{
    self, size_hint, Deserializer, Expected, IntoDeserializer, SeqAccess, Visitor,
};
use crate::ser;
macro_rules! impl_copy_clone {
    ($ty:ident $(<$lifetime:tt >)*) => {
        impl <$($lifetime,)* E > Copy for $ty <$($lifetime,)* E > {} impl <$($lifetime,)*
        E > Clone for $ty <$($lifetime,)* E > { fn clone(& self) -> Self { * self } }
    };
}
impl_copy_clone!(UnitDeserializer);
macro_rules! primitive_deserializer {
    ($ty:ty, $doc:tt, $name:ident, $method:ident $($cast:tt)*) => {
        #[doc = "A deserializer holding"] #[doc = $doc] pub struct $name < E > { value :
        $ty, marker : PhantomData < E > } impl_copy_clone!($name); impl <'de, E >
        IntoDeserializer <'de, E > for $ty where E : de::Error, { type Deserializer =
        $name < E >; fn into_deserializer(self) -> $name < E > { $name ::new(self) } }
        impl < E > $name < E > { #[allow(missing_docs)] pub fn new(value : $ty) -> Self {
        $name { value, marker : PhantomData, } } } impl <'de, E > de::Deserializer <'de >
        for $name < E > where E : de::Error, { type Error = E;
        forward_to_deserialize_any! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32
        f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq
        tuple tuple_struct map struct enum identifier ignored_any } fn deserialize_any <
        V > (self, visitor : V) -> Result < V::Value, Self::Error > where V : de::Visitor
        <'de >, { visitor.$method (self.value $($cast)*) } } impl <'de, E >
        IntoDeserializer <'de, E > for $name < E > where E : de::Error, { type
        Deserializer = Self; fn into_deserializer(self) -> Self { self } } impl < E >
        Debug for $name < E > { fn fmt(& self, formatter : & mut fmt::Formatter) ->
        fmt::Result { formatter.debug_struct(stringify!($name)).field("value", & self
        .value).finish() } }
    };
}
primitive_deserializer!(bool, "a `bool`.", BoolDeserializer, visit_bool);
primitive_deserializer!(i8, "an `i8`.", I8Deserializer, visit_i8);
primitive_deserializer!(i16, "an `i16`.", I16Deserializer, visit_i16);
primitive_deserializer!(i32, "an `i32`.", I32Deserializer, visit_i32);
primitive_deserializer!(i64, "an `i64`.", I64Deserializer, visit_i64);
primitive_deserializer!(i128, "an `i128`.", I128Deserializer, visit_i128);
primitive_deserializer!(isize, "an `isize`.", IsizeDeserializer, visit_i64 as i64);
primitive_deserializer!(u8, "a `u8`.", U8Deserializer, visit_u8);
primitive_deserializer!(u16, "a `u16`.", U16Deserializer, visit_u16);
primitive_deserializer!(u64, "a `u64`.", U64Deserializer, visit_u64);
primitive_deserializer!(u128, "a `u128`.", U128Deserializer, visit_u128);
primitive_deserializer!(usize, "a `usize`.", UsizeDeserializer, visit_u64 as u64);
primitive_deserializer!(f32, "an `f32`.", F32Deserializer, visit_f32);
primitive_deserializer!(f64, "an `f64`.", F64Deserializer, visit_f64);
primitive_deserializer!(char, "a `char`.", CharDeserializer, visit_char);
impl_copy_clone!(U32Deserializer);
impl_copy_clone!(StrDeserializer <'de >);
impl_copy_clone!(BorrowedStrDeserializer <'de >);
impl_copy_clone!(BytesDeserializer <'a >);
impl_copy_clone!(BorrowedBytesDeserializer <'de >);
pub trait Deserializer<'de>: Sized {
    type Error: Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = visitor;
        Err(Error::custom("i128 is not supported"))
    }
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = visitor;
        Err(Error::custom("u128 is not supported"))
    }
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    #[inline]
    fn is_human_readable(&self) -> bool;
    #[cfg(all(not(no_serde_derive), any(feature = "std", feature = "alloc")))]
    fn __deserialize_content<V>(
        self,
        _: crate::actually_private::T,
        visitor: V,
    ) -> Result<crate::__private::de::Content<'de>, Self::Error>
    where
        V: Visitor<'de, Value = crate::__private::de::Content<'de>>,
    {
        self.deserialize_any(visitor)
    }
}
pub trait IntoDeserializer<'de, E: Error = value::Error> {
    type Deserializer: Deserializer<'de, Error = E>;
    fn into_deserializer(self) -> Self::Deserializer;
}
pub trait EnumAccess<'de>: Sized {
    type Error: Error;
    type Variant: VariantAccess<'de, Error = Self::Error>;
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>;
    #[inline]
    fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>
    where
        V: Deserialize<'de>,
    {
        self.variant_seed(PhantomData)
    }
}
#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
pub struct StringDeserializer<E> {
    value: String,
    marker: PhantomData<E>,
}
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de, E> de::Deserializer<'de> for StringDeserializer<E>
where
    E: de::Error,
{
    type Error = E;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(self.value)
    }
    fn deserialize_enum<V>(
        self,
        name: &str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let _ = name;
        let _ = variants;
        visitor.visit_enum(self)
    }
}
