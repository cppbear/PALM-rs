use crate::lib::*;
use crate::de::value::{BorrowedBytesDeserializer, BytesDeserializer};
use crate::de::{
    Deserialize, DeserializeSeed, Deserializer, EnumAccess, Error, IntoDeserializer,
    VariantAccess, Visitor,
};
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::de::{MapAccess, Unexpected};
#[cfg(any(feature = "std", feature = "alloc"))]
pub use self::content::{
    Content, ContentDeserializer, ContentRefDeserializer, EnumDeserializer,
    InternallyTaggedUnitVisitor, TagContentOtherField, TagContentOtherFieldVisitor,
    TagOrContentField, TagOrContentFieldVisitor, TaggedContentVisitor,
    UntaggedUnitVisitor,
};
pub use crate::seed::InPlaceSeed;
#[cfg(any(feature = "std", feature = "alloc"))]
macro_rules! forward_to_deserialize_other {
    ($($func:ident ($($arg:ty),*))*) => {
        $(fn $func < V > (self, $(_ : $arg,)* _visitor : V) -> Result < V::Value,
        Self::Error > where V : Visitor <'de >, { Self::deserialize_other() })*
    };
}
pub trait IdentifierDeserializer<'de, E: Error> {
    type Deserializer: Deserializer<'de, Error = E>;
    fn from(self) -> Self::Deserializer;
}
pub struct Borrowed<'de, T: 'de + ?Sized>(pub &'de T);
pub struct T;
pub struct BorrowedStrDeserializer<'de, E> {
    value: &'de str,
    marker: PhantomData<E>,
}
#[derive(Clone, PartialEq)]
pub struct Error {
    err: ErrorImpl,
}
pub struct BorrowedBytesDeserializer<'de, E> {
    value: &'de [u8],
    marker: PhantomData<E>,
}
#[derive(Debug)]
pub struct Error;
impl<'de, E> IdentifierDeserializer<'de, E> for Borrowed<'de, [u8]>
where
    E: Error,
{
    type Deserializer = BorrowedBytesDeserializer<'de, E>;
    fn from(self) -> Self::Deserializer {
        BorrowedBytesDeserializer::new(self.0)
    }
}
