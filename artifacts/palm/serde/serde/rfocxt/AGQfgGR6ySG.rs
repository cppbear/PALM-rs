pub type First<T> = <T as Pair>::First;
pub type Second<T> = <T as Pair>::Second;
use crate::lib::*;
use crate::de::{
    self, DeserializeSeed, Deserializer, MapAccess, Unexpected, VariantAccess, Visitor,
};
pub struct UnitOnly<E> {
    marker: PhantomData<E>,
}
pub struct T;
pub fn unit_only<T, E>(t: T) -> (T, UnitOnly<E>) {
    (t, UnitOnly { marker: PhantomData })
}
