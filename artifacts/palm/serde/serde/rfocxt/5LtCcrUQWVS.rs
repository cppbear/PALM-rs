use crate::lib::*;
use crate::ser::{self, Impossible, Serialize, SerializeMap, SerializeStruct, Serializer};
#[cfg(any(feature = "std", feature = "alloc"))]
use self::content::{
    Content, ContentSerializer, SerializeStructVariantAsMapValue,
    SerializeTupleVariantAsMapValue,
};
pub struct T;
pub fn constrain<T: ?Sized>(t: &T) -> &T {
    t
}
