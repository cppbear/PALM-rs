use crate::lib::*;
pub use self::ignored_any::IgnoredAny;
#[cfg(all(not(feature = "std"), no_core_error))]
pub use crate::std_error::Error as StdError;
#[cfg(not(any(feature = "std", no_core_error)))]
pub use core::error::Error as StdError;
#[cfg(feature = "std")]
pub use std::error::Error as StdError;
macro_rules! declare_error_trait {
    (Error : Sized $(+ $($supertrait:ident)::+)*) => {
        #[doc =
        " The `Error` trait allows `Deserialize` implementations to create descriptive"]
        #[doc = " error messages belonging to the `Deserializer` against which they are"]
        #[doc = " currently running."] #[doc = ""] #[doc =
        " Every `Deserializer` declares an `Error` type that encompasses both"] #[doc =
        " general-purpose deserialization errors as well as errors specific to the"]
        #[doc = " particular deserialization format. For example the `Error` type of"]
        #[doc =
        " `serde_json` can represent errors like an invalid JSON escape sequence or an"]
        #[doc =
        " unterminated string literal, in addition to the error cases that are part of"]
        #[doc = " this trait."] #[doc = ""] #[doc =
        " Most deserializers should only need to provide the `Error::custom` method"]
        #[doc = " and inherit the default behavior for the other methods."] #[doc = ""]
        #[doc = " # Example implementation"] #[doc = ""] #[doc =
        " The [example data format] presented on the website shows an error"] #[doc =
        " type appropriate for a basic JSON data format."] #[doc = ""] #[doc =
        " [example data format]: https://serde.rs/data-format.html"] pub trait Error :
        Sized $(+ $($supertrait)::+)* { #[doc =
        " Raised when there is general error when deserializing a type."] #[doc = ""]
        #[doc =
        " The message should not be capitalized and should not end with a period."] #[doc
        = ""] #[doc = " ```edition2021"] #[doc = " # use std::str::FromStr;"] #[doc =
        " #"] #[doc = " # struct IpAddr;"] #[doc = " #"] #[doc =
        " # impl FromStr for IpAddr {"] #[doc = " #     type Err = String;"] #[doc =
        " #"] #[doc = " #     fn from_str(_: &str) -> Result<Self, String> {"] #[doc =
        " #         unimplemented!()"] #[doc = " #     }"] #[doc = " # }"] #[doc = " #"]
        #[doc = " use serde::de::{self, Deserialize, Deserializer};"] #[doc = ""] #[doc =
        " impl<'de> Deserialize<'de> for IpAddr {"] #[doc =
        "     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>"] #[doc =
        "     where"] #[doc = "         D: Deserializer<'de>,"] #[doc = "     {"] #[doc =
        "         let s = String::deserialize(deserializer)?;"] #[doc =
        "         s.parse().map_err(de::Error::custom)"] #[doc = "     }"] #[doc = " }"]
        #[doc = " ```"] fn custom < T > (msg : T) -> Self where T : Display; #[doc =
        " Raised when a `Deserialize` receives a type different from what it was"] #[doc
        = " expecting."] #[doc = ""] #[doc =
        " The `unexp` argument provides information about what type was received."] #[doc
        = " This is the type that was present in the input file or other source data"]
        #[doc = " of the Deserializer."] #[doc = ""] #[doc =
        " The `exp` argument provides information about what type was being"] #[doc =
        " expected. This is the type that is written in the program."] #[doc = ""] #[doc
        = " For example if we try to deserialize a String out of a JSON file"] #[doc =
        " containing an integer, the unexpected type is the integer and the"] #[doc =
        " expected type is the string."] #[cold] fn invalid_type(unexp : Unexpected, exp
        : & dyn Expected) -> Self {
        Error::custom(format_args!("invalid type: {}, expected {}", unexp, exp)) } #[doc
        = " Raised when a `Deserialize` receives a value of the right type but that"]
        #[doc = " is wrong for some other reason."] #[doc = ""] #[doc =
        " The `unexp` argument provides information about what value was received."]
        #[doc = " This is the value that was present in the input file or other source"]
        #[doc = " data of the Deserializer."] #[doc = ""] #[doc =
        " The `exp` argument provides information about what value was being"] #[doc =
        " expected. This is the type that is written in the program."] #[doc = ""] #[doc
        = " For example if we try to deserialize a String out of some binary data"] #[doc
        = " that is not valid UTF-8, the unexpected value is the bytes and the"] #[doc =
        " expected value is a string."] #[cold] fn invalid_value(unexp : Unexpected, exp
        : & dyn Expected) -> Self {
        Error::custom(format_args!("invalid value: {}, expected {}", unexp, exp)) } #[doc
        = " Raised when deserializing a sequence or map and the input data contains"]
        #[doc = " too many or too few elements."] #[doc = ""] #[doc =
        " The `len` argument is the number of elements encountered. The sequence"] #[doc
        = " or map may have expected more arguments or fewer arguments."] #[doc = ""]
        #[doc = " The `exp` argument provides information about what data was being"]
        #[doc = " expected. For example `exp` might say that a tuple of size 6 was"]
        #[doc = " expected."] #[cold] fn invalid_length(len : usize, exp : & dyn
        Expected) -> Self { Error::custom(format_args!("invalid length {}, expected {}",
        len, exp)) } #[doc =
        " Raised when a `Deserialize` enum type received a variant with an"] #[doc =
        " unrecognized name."] #[cold] fn unknown_variant(variant : & str, expected :
        &'static [&'static str]) -> Self { if expected.is_empty() {
        Error::custom(format_args!("unknown variant `{}`, there are no variants",
        variant)) } else {
        Error::custom(format_args!("unknown variant `{}`, expected {}", variant, OneOf {
        names : expected })) } } #[doc =
        " Raised when a `Deserialize` struct type received a field with an"] #[doc =
        " unrecognized name."] #[cold] fn unknown_field(field : & str, expected :
        &'static [&'static str]) -> Self { if expected.is_empty() {
        Error::custom(format_args!("unknown field `{}`, there are no fields", field)) }
        else { Error::custom(format_args!("unknown field `{}`, expected {}", field, OneOf
        { names : expected })) } } #[doc =
        " Raised when a `Deserialize` struct type expected to receive a required"] #[doc
        = " field with a particular name but that field was not present in the"] #[doc =
        " input."] #[cold] fn missing_field(field : &'static str) -> Self {
        Error::custom(format_args!("missing field `{}`", field)) } #[doc =
        " Raised when a `Deserialize` struct type received more than one of the"] #[doc =
        " same field."] #[cold] fn duplicate_field(field : &'static str) -> Self {
        Error::custom(format_args!("duplicate field `{}`", field)) } }
    };
}
#[cfg(feature = "std")]
declare_error_trait!(Error : Sized + StdError);
#[cfg(not(feature = "std"))]
declare_error_trait!(Error : Sized + Debug + Display);
pub trait SeqAccess<'de> {
    type Error: Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>;
    #[inline]
    fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
    where
        T: Deserialize<'de>,
    {
        self.next_element_seed(PhantomData)
    }
    #[inline]
    fn size_hint(&self) -> Option<usize> {
        None
    }
}
pub trait Serializer: Sized {
    type Ok;
    type Error: Error;
    type SerializeSeq: SerializeSeq<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTuple: SerializeTuple<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTupleStruct: SerializeTupleStruct<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTupleVariant: SerializeTupleVariant<
            Ok = Self::Ok,
            Error = Self::Error,
        >;
    type SerializeMap: SerializeMap<Ok = Self::Ok, Error = Self::Error>;
    type SerializeStruct: SerializeStruct<Ok = Self::Ok, Error = Self::Error>;
    type SerializeStructVariant: SerializeStructVariant<
            Ok = Self::Ok,
            Error = Self::Error,
        >;
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>;
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error>;
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error>;
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>;
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("i128 is not supported"))
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error>;
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error>;
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error>;
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>;
    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("u128 is not supported"))
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error>;
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>;
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>;
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>;
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error>;
    fn serialize_none(self) -> Result<Self::Ok, Self::Error>;
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize;
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error>;
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error>;
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error>;
    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize;
    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize;
    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Self::Error>;
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error>;
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error>;
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error>;
    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeMap, Self::Error>;
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>;
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error>;
    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        let mut iter = iter.into_iter();
        let mut serializer = tri!(self.serialize_seq(iterator_len_hint(& iter)));
        tri!(iter.try_for_each(| item | serializer.serialize_element(& item)));
        serializer.end()
    }
    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        let mut iter = iter.into_iter();
        let mut serializer = tri!(self.serialize_map(iterator_len_hint(& iter)));
        tri!(
            iter.try_for_each(| (key, value) | serializer.serialize_entry(& key, &
            value))
        );
        serializer.end()
    }
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Display,
    {
        self.serialize_str(&value.to_string())
    }
    #[cfg(not(any(feature = "std", feature = "alloc")))]
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Display;
    #[inline]
    fn is_human_readable(&self) -> bool;
}
pub trait Pair {
    type First;
    type Second;
    fn split(self) -> (Self::First, Self::Second);
}
pub trait MapAccess<'de> {
    type Error: Error;
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>;
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>;
    #[inline]
    fn next_entry_seed<K, V>(
        &mut self,
        kseed: K,
        vseed: V,
    ) -> Result<Option<(K::Value, V::Value)>, Self::Error>
    where
        K: DeserializeSeed<'de>,
        V: DeserializeSeed<'de>,
    {
        match tri!(self.next_key_seed(kseed)) {
            Some(key) => {
                let value = tri!(self.next_value_seed(vseed));
                Ok(Some((key, value)))
            }
            None => Ok(None),
        }
    }
    #[inline]
    fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
    where
        K: Deserialize<'de>,
    {
        self.next_key_seed(PhantomData)
    }
    #[inline]
    fn next_value<V>(&mut self) -> Result<V, Self::Error>
    where
        V: Deserialize<'de>,
    {
        self.next_value_seed(PhantomData)
    }
    #[inline]
    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        self.next_entry_seed(PhantomData, PhantomData)
    }
    #[inline]
    fn size_hint(&self) -> Option<usize>;
}
#[cfg_attr(
    not(no_diagnostic_namespace),
    diagnostic::on_unimplemented(
        note = "for local types consider adding `#[derive(serde::Deserialize)]` to your `{Self}` type",
        note = "for types from other crates check whether the crate offers a `serde` feature flag",
    )
)]
pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>;
}
pub trait IdentifierDeserializer<'de, E: Error> {
    type Deserializer: Deserializer<'de, Error = E>;
    fn from(self) -> Self::Deserializer;
}
pub trait Expected {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result;
}
#[cfg_attr(
    not(no_diagnostic_namespace),
    diagnostic::on_unimplemented(
        note = "for local types consider adding `#[derive(serde::Serialize)]` to your `{Self}` type",
        note = "for types from other crates check whether the crate offers a `serde` feature flag",
    )
)]
pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
pub trait DeserializeSeed<'de>: Sized {
    type Value;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>;
}
pub trait IntoDeserializer<'de, E: Error = value::Error> {
    type Deserializer: Deserializer<'de, Error = E>;
    fn into_deserializer(self) -> Self::Deserializer;
}
#[derive(Debug)]
pub struct Error;
pub struct T;
#[derive(Clone, PartialEq)]
pub struct Error {
    err: ErrorImpl,
}
impl<'de, A> SeqAccess<'de> for &mut A
where
    A: ?Sized + SeqAccess<'de>,
{
    type Error = A::Error;
    #[inline]
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        (**self).next_element_seed(seed)
    }
    #[inline]
    fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
    where
        T: Deserialize<'de>,
    {
        (**self).next_element()
    }
    #[inline]
    fn size_hint(&self) -> Option<usize> {}
}
