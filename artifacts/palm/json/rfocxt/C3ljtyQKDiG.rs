use crate::error::{Error, ErrorCode, Result};
use crate::io;
use alloc::string::String;
#[cfg(feature = "raw_value")]
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::{self, Display};
use core::num::FpCategory;
use serde::ser::{self, Impossible, Serialize};
static ESCAPE: [u8; 256] = [
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    BB,
    TT,
    NN,
    UU,
    FF,
    RR,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    __,
    __,
    QU,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    BS,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
];
const BB: u8 = b'b';
const TT: u8 = b't';
const NN: u8 = b'n';
const FF: u8 = b'f';
const RR: u8 = b'r';
const QU: u8 = b'"';
const BS: u8 = b'\\';
const UU: u8 = b'u';
const __: u8 = 0;
struct MapKeySerializer<'a, W: 'a, F: 'a> {
    ser: &'a mut Serializer<W, F>,
}
pub struct SerializeStructVariant {
    name: String,
    map: Map<String, Value>,
}
struct MapKeySerializer;
pub struct Error;
pub struct SerializeTupleVariant {
    name: String,
    vec: Vec<Value>,
}
pub struct Serializer;
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct Serializer<W, F = CompactFormatter> {
    writer: W,
    formatter: F,
}
pub struct Error {
    /// This `Box` allows us to keep the size of `Error` as small as possible. A
    /// larger `Error` type was substantially slower due to all the functions
    /// that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>,
}
pub enum Compound<'a, W: 'a, F: 'a> {
    Map { ser: &'a mut Serializer<W, F>, state: State },
    #[cfg(feature = "arbitrary_precision")]
    Number { ser: &'a mut Serializer<W, F> },
    #[cfg(feature = "raw_value")]
    RawValue { ser: &'a mut Serializer<W, F> },
}
pub enum SerializeMap {
    Map { map: Map<String, Value>, next_key: Option<String> },
    #[cfg(feature = "arbitrary_precision")]
    Number { out_value: Option<Value> },
    #[cfg(feature = "raw_value")]
    RawValue { out_value: Option<Value> },
}
#[derive(Eq, PartialEq)]
pub enum State {
    Empty,
    First,
    Rest,
}
impl<'a, W, F> ser::SerializeMap for Compound<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;
    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Compound::Map { ser, state } => {
                tri!(
                    ser.formatter.begin_object_key(& mut ser.writer, * state ==
                    State::First).map_err(Error::io)
                );
                *state = State::Rest;
                tri!(key.serialize(MapKeySerializer { ser : * ser }));
                ser.formatter.end_object_key(&mut ser.writer).map_err(Error::io)
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => unreachable!(),
        }
    }
    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {}
    #[inline]
    fn end(self) -> Result<()> {}
}
