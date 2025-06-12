pub type First<T> = <T as Pair>::First;
pub type Second<T> = <T as Pair>::Second;
use crate::lib::*;
use crate::de::{
    self, DeserializeSeed, Deserializer, MapAccess, Unexpected, VariantAccess, Visitor,
};
pub struct MapAsEnum<A> {
    map: A,
}
pub fn map_as_enum<A>(map: A) -> MapAsEnum<A> {
    MapAsEnum { map }
}
