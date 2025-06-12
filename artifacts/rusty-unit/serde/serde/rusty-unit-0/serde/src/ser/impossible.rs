//! This module contains `Impossible` serializer and its implementations.

use crate::lib::*;

use crate::ser::{
    self, Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
};

/// Helper type for implementing a `Serializer` that does not support
/// serializing one of the compound types.
///
/// This type cannot be instantiated, but implements every one of the traits
/// corresponding to the [`Serializer`] compound types: [`SerializeSeq`],
/// [`SerializeTuple`], [`SerializeTupleStruct`], [`SerializeTupleVariant`],
/// [`SerializeMap`], [`SerializeStruct`], and [`SerializeStructVariant`].
///
/// ```edition2021
/// # use serde::ser::{Serializer, Impossible};
/// # use serde::__private::doc::Error;
/// #
/// # struct MySerializer;
/// #
/// impl Serializer for MySerializer {
///     type Ok = ();
///     type Error = Error;
///
///     type SerializeSeq = Impossible<(), Error>;
///     /* other associated types */
///
///     /// This data format does not support serializing sequences.
///     fn serialize_seq(self,
///                      len: Option<usize>)
///                      -> Result<Self::SerializeSeq, Error> {
///         // Given Impossible cannot be instantiated, the only
///         // thing we can do here is to return an error.
/// #         stringify! {
///         Err(...)
/// #         };
/// #         unimplemented!()
///     }
///
///     /* other Serializer methods */
/// #     serde::__serialize_unimplemented! {
/// #         bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str bytes none some
/// #         unit unit_struct unit_variant newtype_struct newtype_variant
/// #         tuple tuple_struct tuple_variant map struct struct_variant
/// #     }
/// }
/// ```
///
/// [`Serializer`]: crate::Serializer
/// [`SerializeSeq`]: crate::ser::SerializeSeq
/// [`SerializeTuple`]: crate::ser::SerializeTuple
/// [`SerializeTupleStruct`]: crate::ser::SerializeTupleStruct
/// [`SerializeTupleVariant`]: crate::ser::SerializeTupleVariant
/// [`SerializeMap`]: crate::ser::SerializeMap
/// [`SerializeStruct`]: crate::ser::SerializeStruct
/// [`SerializeStructVariant`]: crate::ser::SerializeStructVariant
pub struct Impossible<Ok, Error> {
    void: Void,
    ok: PhantomData<Ok>,
    error: PhantomData<Error>,
}

enum Void {}

impl<Ok, Error> SerializeSeq for Impossible<Ok, Error>
where
    Error: ser::Error,
{
    type Ok = Ok;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = value;
        match self.void {}
    }

    fn end(self) -> Result<Ok, Error> {
        match self.void {}
    }
}

impl<Ok, Error> SerializeTuple for Impossible<Ok, Error>
where
    Error: ser::Error,
{
    type Ok = Ok;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = value;
        match self.void {}
    }

    fn end(self) -> Result<Ok, Error> {
        match self.void {}
    }
}

impl<Ok, Error> SerializeTupleStruct for Impossible<Ok, Error>
where
    Error: ser::Error,
{
    type Ok = Ok;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = value;
        match self.void {}
    }

    fn end(self) -> Result<Ok, Error> {
        match self.void {}
    }
}

impl<Ok, Error> SerializeTupleVariant for Impossible<Ok, Error>
where
    Error: ser::Error,
{
    type Ok = Ok;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = value;
        match self.void {}
    }

    fn end(self) -> Result<Ok, Error> {
        match self.void {}
    }
}

impl<Ok, Error> SerializeMap for Impossible<Ok, Error>
where
    Error: ser::Error,
{
    type Ok = Ok;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = key;
        match self.void {}
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = value;
        match self.void {}
    }

    fn end(self) -> Result<Ok, Error> {
        match self.void {}
    }
}

impl<Ok, Error> SerializeStruct for Impossible<Ok, Error>
where
    Error: ser::Error,
{
    type Ok = Ok;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = key;
        let _ = value;
        match self.void {}
    }

    fn end(self) -> Result<Ok, Error> {
        match self.void {}
    }
}

impl<Ok, Error> SerializeStructVariant for Impossible<Ok, Error>
where
    Error: ser::Error,
{
    type Ok = Ok;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = key;
        let _ = value;
        match self.void {}
    }

    fn end(self) -> Result<Ok, Error> {
        match self.void {}
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::clone::Clone;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_24() {
    rusty_monitor::set_test_id(24);
    let mut u128_0: u128 = 4228u128;
    let mut isize_0: isize = 11296isize;
    let mut usize_0: usize = 9591usize;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_1: isize = 12407isize;
    let mut str_1: &str = "NT3UXt";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "PbdAfJOHjoXYU9p6kg";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_0: u64 = 6503u64;
    let mut f32_0: f32 = -5305.392867f32;
    let mut f32deserializer_0: crate::de::value::F32Deserializer<isize> = crate::de::value::F32Deserializer::new(f32_0);
    let mut f32deserializer_0_ref_0: &crate::de::value::F32Deserializer<isize> = &mut f32deserializer_0;
    let mut vec_0: std::vec::Vec<(__private::ser::content::Content, __private::ser::content::Content)> = std::vec::Vec::new();
    let mut vec_1: std::vec::Vec<(&str, __private::ser::content::Content)> = std::vec::Vec::new();
    let mut str_3: &str = "X57ig6u7vmFZuDhH8RU";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u32_0: u32 = 1399u32;
    let mut str_4: &str = "AWCpAL9yy28jJNdPZT";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut content_0: __private::ser::content::Content = crate::__private::ser::content::Content::StructVariant(str_4_ref_0, u32_0, str_3_ref_0, vec_1);
    let mut tagorcontentfield_0: __private::de::content::TagOrContentField = crate::__private::de::content::TagOrContentField::Content;
    let mut content_1: __private::ser::content::Content = crate::__private::ser::content::Content::Map(vec_0);
    let mut f32deserializer_1: crate::de::value::F32Deserializer<isize> = crate::de::value::F32Deserializer::clone(f32deserializer_0_ref_0);
    let mut tagcontentotherfield_0: __private::de::content::TagContentOtherField = crate::__private::de::content::TagContentOtherField::Other;
    let mut content_2: __private::ser::content::Content = crate::__private::ser::content::Content::U64(u64_0);
    let mut f32deserializer_1_ref_0: &crate::de::value::F32Deserializer<isize> = &mut f32deserializer_1;
    let mut serializestructvariantasmapvalue_0: crate::__private::ser::content::SerializeStructVariantAsMapValue<isize> = crate::__private::ser::content::SerializeStructVariantAsMapValue::new(isize_1, str_0_ref_0, usize_0);
    let mut tuple_0: (isize, crate::de::value::private::UnitOnly<isize>) = crate::de::value::private::unit_only(isize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_30() {
    rusty_monitor::set_test_id(30);
    let mut usize_0: usize = 4553usize;
    let mut seqaccessdeserializer_0: crate::de::value::SeqAccessDeserializer<usize> = crate::de::value::SeqAccessDeserializer::new(usize_0);
    let mut seqaccessdeserializer_0_ref_0: &crate::de::value::SeqAccessDeserializer<usize> = &mut seqaccessdeserializer_0;
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 3213i64;
    let mut i64deserializer_0: crate::de::value::I64Deserializer<isize> = crate::de::value::I64Deserializer::new(i64_0);
    let mut i64deserializer_0_ref_0: &crate::de::value::I64Deserializer<isize> = &mut i64deserializer_0;
    let mut i64_1: i64 = -5656i64;
    let mut isize_0: isize = -9951isize;
    let mut str_0: &str = "vnpIC4UepqWO";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut enumaccessdeserializer_0: crate::de::value::EnumAccessDeserializer<isize> = crate::de::value::EnumAccessDeserializer::new(isize_0);
    let mut i64deserializer_1: crate::de::value::I64Deserializer<isize> = crate::de::value::I64Deserializer::clone(i64deserializer_0_ref_0);
    let mut i64deserializer_1_ref_0: &crate::de::value::I64Deserializer<isize> = &mut i64deserializer_1;
    let mut i64deserializer_2: crate::de::value::I64Deserializer<isize> = crate::de::value::I64Deserializer::clone(i64deserializer_1_ref_0);
    let mut enumaccessdeserializer_0_ref_0: &crate::de::value::EnumAccessDeserializer<isize> = &mut enumaccessdeserializer_0;
    let mut enumaccessdeserializer_1: crate::de::value::EnumAccessDeserializer<isize> = crate::de::value::EnumAccessDeserializer::clone(enumaccessdeserializer_0_ref_0);
    let mut tagcontentotherfield_0: __private::de::content::TagContentOtherField = crate::__private::de::content::TagContentOtherField::Content;
    let mut i64deserializer_2_ref_0: &crate::de::value::I64Deserializer<isize> = &mut i64deserializer_2;
    let mut i64deserializer_3: crate::de::value::I64Deserializer<isize> = crate::de::value::I64Deserializer::clone(i64deserializer_2_ref_0);
    let mut enumaccessdeserializer_1_ref_0: &crate::de::value::EnumAccessDeserializer<isize> = &mut enumaccessdeserializer_1;
    let mut enumaccessdeserializer_2: crate::de::value::EnumAccessDeserializer<isize> = crate::de::value::EnumAccessDeserializer::clone(enumaccessdeserializer_1_ref_0);
    let mut enumaccessdeserializer_2_ref_0: &crate::de::value::EnumAccessDeserializer<isize> = &mut enumaccessdeserializer_2;
    let mut enumaccessdeserializer_3: crate::de::value::EnumAccessDeserializer<isize> = crate::de::value::EnumAccessDeserializer::clone(enumaccessdeserializer_2_ref_0);
    let mut booldeserializer_0: crate::de::value::BoolDeserializer<isize> = crate::de::value::BoolDeserializer::new(bool_0);
    let mut enumaccessdeserializer_3_ref_0: &crate::de::value::EnumAccessDeserializer<isize> = &mut enumaccessdeserializer_3;
    let mut enumaccessdeserializer_4: crate::de::value::EnumAccessDeserializer<isize> = crate::de::value::EnumAccessDeserializer::clone(enumaccessdeserializer_3_ref_0);
    let mut enumaccessdeserializer_4_ref_0: &crate::de::value::EnumAccessDeserializer<isize> = &mut enumaccessdeserializer_4;
    let mut enumaccessdeserializer_5: crate::de::value::EnumAccessDeserializer<isize> = crate::de::value::EnumAccessDeserializer::clone(enumaccessdeserializer_4_ref_0);
    let mut seqaccessdeserializer_1: crate::de::value::SeqAccessDeserializer<usize> = crate::de::value::SeqAccessDeserializer::clone(seqaccessdeserializer_0_ref_0);
    panic!("From RustyUnit with love");
}
}