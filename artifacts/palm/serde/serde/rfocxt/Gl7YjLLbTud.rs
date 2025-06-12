use crate::lib::*;
use crate::ser::{self, Serialize, Serializer};
pub trait SerializeSeq {
    type Ok;
    type Error: Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}
pub struct SerializeSeq<E> {
    elements: Vec<Content>,
    error: PhantomData<E>,
}
#[derive(Debug, Clone)]
pub enum Content<'de> {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Char(char),
    String(String),
    Str(&'de str),
    ByteBuf(Vec<u8>),
    Bytes(&'de [u8]),
    None,
    Some(Box<Content<'de>>),
    Unit,
    Newtype(Box<Content<'de>>),
    Seq(Vec<Content<'de>>),
    Map(Vec<(Content<'de>, Content<'de>)>),
}
pub enum Content {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Char(char),
    String(String),
    Bytes(Vec<u8>),
    None,
    Some(Box<Content>),
    Unit,
    UnitStruct(&'static str),
    UnitVariant(&'static str, u32, &'static str),
    NewtypeStruct(&'static str, Box<Content>),
    NewtypeVariant(&'static str, u32, &'static str, Box<Content>),
    Seq(Vec<Content>),
    Tuple(Vec<Content>),
    TupleStruct(&'static str, Vec<Content>),
    TupleVariant(&'static str, u32, &'static str, Vec<Content>),
    Map(Vec<(Content, Content)>),
    Struct(&'static str, Vec<(&'static str, Content)>),
    StructVariant(&'static str, u32, &'static str, Vec<(&'static str, Content)>),
}
impl<E> ser::SerializeSeq for SerializeSeq<E>
where
    E: ser::Error,
{
    type Ok = Content;
    type Error = E;
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), E>
    where
        T: ?Sized + Serialize,
    {}
    fn end(self) -> Result<Content, E> {
        Ok(Content::Seq(self.elements))
    }
}
