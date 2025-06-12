use crate::lib::*;
use crate::ser::{self, Impossible, Serialize, SerializeMap, SerializeStruct, Serializer};
#[cfg(any(feature = "std", feature = "alloc"))]
use self::content::{
    Content, ContentSerializer, SerializeStructVariantAsMapValue,
    SerializeTupleVariantAsMapValue,
};
struct TaggedSerializer<S> {
    type_ident: &'static str,
    variant_ident: &'static str,
    tag: &'static str,
    variant_name: &'static str,
    delegate: S,
}
pub struct T;
pub fn serialize_tagged_newtype<S, T>(
    serializer: S,
    type_ident: &'static str,
    variant_ident: &'static str,
    tag: &'static str,
    variant_name: &'static str,
    value: &T,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    value
        .serialize(TaggedSerializer {
            type_ident,
            variant_ident,
            tag,
            variant_name,
            delegate: serializer,
        })
}
