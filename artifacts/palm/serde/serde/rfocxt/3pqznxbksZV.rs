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
pub trait DeserializeSeed<'de>: Sized {
    type Value;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>;
}
pub struct AdjacentlyTaggedEnumVariantSeed<F> {
    pub enum_name: &'static str,
    pub variants: &'static [&'static str],
    pub fields_enum: PhantomData<F>,
}
pub struct AdjacentlyTaggedEnumVariantVisitor<F> {
    enum_name: &'static str,
    fields_enum: PhantomData<F>,
}
impl<'de, F> DeserializeSeed<'de> for AdjacentlyTaggedEnumVariantSeed<F>
where
    F: Deserialize<'de>,
{
    type Value = F;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_enum(
                self.enum_name,
                self.variants,
                AdjacentlyTaggedEnumVariantVisitor {
                    enum_name: self.enum_name,
                    fields_enum: PhantomData,
                },
            )
    }
}
