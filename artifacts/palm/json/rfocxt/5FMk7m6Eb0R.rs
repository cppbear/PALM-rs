#[cfg(feature = "arbitrary_precision")]
type N = String;
use crate::de::ParserNumber;
use crate::error::Error;
#[cfg(feature = "arbitrary_precision")]
use crate::error::ErrorCode;
#[cfg(feature = "arbitrary_precision")]
use alloc::borrow::ToOwned;
#[cfg(feature = "arbitrary_precision")]
use alloc::string::{String, ToString};
use core::fmt::{self, Debug, Display};
#[cfg(not(feature = "arbitrary_precision"))]
use core::hash::{Hash, Hasher};
use serde::de::{self, Unexpected, Visitor};
#[cfg(feature = "arbitrary_precision")]
use serde::de::{IntoDeserializer, MapAccess};
use serde::{
    forward_to_deserialize_any, Deserialize, Deserializer, Serialize, Serializer,
};
#[cfg(feature = "arbitrary_precision")]
pub(crate) const TOKEN: &str = "$serde_json::private::Number";
macro_rules! deserialize_any {
    (@ expand[$($num_string:tt)*]) => {
        #[cfg(not(feature = "arbitrary_precision"))] fn deserialize_any < V > (self,
        visitor : V) -> Result < V::Value, Error > where V : Visitor <'de >, { match self
        .n { N::PosInt(u) => visitor.visit_u64(u), N::NegInt(i) => visitor.visit_i64(i),
        N::Float(f) => visitor.visit_f64(f), } } #[cfg(feature = "arbitrary_precision")]
        fn deserialize_any < V > (self, visitor : V) -> Result < V::Value, Error > where
        V : Visitor <'de > { if let Some(u) = self.as_u64() { return visitor
        .visit_u64(u); } else if let Some(i) = self.as_i64() { return visitor
        .visit_i64(i); } else if let Some(u) = self.as_u128() { return visitor
        .visit_u128(u); } else if let Some(i) = self.as_i128() { return visitor
        .visit_i128(i); } else if let Some(f) = self.as_f64() { if ryu::Buffer::new()
        .format_finite(f) == self.n || f.to_string() == self.n { return visitor
        .visit_f64(f); } } visitor.visit_map(NumberDeserializer { number : Some(self
        .$($num_string)*), }) }
    };
    (owned) => {
        deserialize_any!(@ expand[n]);
    };
    (ref) => {
        deserialize_any!(@ expand[n.clone()]);
    };
}
macro_rules! deserialize_number {
    ($deserialize:ident => $visit:ident) => {
        #[cfg(not(feature = "arbitrary_precision"))] fn $deserialize < V > (self, visitor
        : V) -> Result < V::Value, Error > where V : Visitor <'de >, { self
        .deserialize_any(visitor) } #[cfg(feature = "arbitrary_precision")] fn
        $deserialize < V > (self, visitor : V) -> Result < V::Value, Error > where V :
        de::Visitor <'de >, { visitor.$visit (tri!(self.n.parse().map_err(| _ |
        invalid_number()))) }
    };
}
macro_rules! impl_from_unsigned {
    ($($ty:ty),*) => {
        $(impl From <$ty > for Number { fn from(u : $ty) -> Self { let n = {
        #[cfg(not(feature = "arbitrary_precision"))] { N::PosInt(u as u64) }
        #[cfg(feature = "arbitrary_precision")] { itoa::Buffer::new().format(u)
        .to_owned() } }; Number { n } } })*
    };
}
macro_rules! impl_from_signed {
    ($($ty:ty),*) => {
        $(impl From <$ty > for Number { fn from(i : $ty) -> Self { let n = {
        #[cfg(not(feature = "arbitrary_precision"))] { if i < 0 { N::NegInt(i as i64) }
        else { N::PosInt(i as u64) } } #[cfg(feature = "arbitrary_precision")] {
        itoa::Buffer::new().format(i).to_owned() } }; Number { n } } })*
    };
}
impl_from_unsigned!(u8, u16, u32, u64, usize);
impl_from_signed!(i8, i16, i32, i64, isize);
#[cfg(feature = "arbitrary_precision")]
impl_from_unsigned!(u128);
#[cfg(feature = "arbitrary_precision")]
impl_from_signed!(i128);
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Number {
    n: N,
}
#[cfg(not(feature = "arbitrary_precision"))]
#[derive(Copy, Clone)]
enum N {
    PosInt(u64),
    /// Always less than zero.
    NegInt(i64),
    /// Always finite.
    Float(f64),
}
impl<'de> Deserialize<'de> for Number {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Number, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumberVisitor;
        impl<'de> Visitor<'de> for NumberVisitor {
            type Value = Number;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a JSON number")
            }
            fn visit_i64<E>(self, value: i64) -> Result<Number, E> {
                Ok(value.into())
            }
            fn visit_i128<E>(self, value: i128) -> Result<Number, E>
            where
                E: de::Error,
            {
                Number::from_i128(value)
                    .ok_or_else(|| de::Error::custom("JSON number out of range"))
            }
            fn visit_u64<E>(self, value: u64) -> Result<Number, E> {
                Ok(value.into())
            }
            fn visit_u128<E>(self, value: u128) -> Result<Number, E>
            where
                E: de::Error,
            {
                Number::from_u128(value)
                    .ok_or_else(|| de::Error::custom("JSON number out of range"))
            }
            fn visit_f64<E>(self, value: f64) -> Result<Number, E>
            where
                E: de::Error,
            {
                Number::from_f64(value)
                    .ok_or_else(|| de::Error::custom("not a JSON number"))
            }
            #[cfg(feature = "arbitrary_precision")]
            fn visit_map<V>(self, mut visitor: V) -> Result<Number, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let value = tri!(visitor.next_key::< NumberKey > ());
                if value.is_none() {
                    return Err(de::Error::invalid_type(Unexpected::Map, &self));
                }
                let v: NumberFromString = tri!(visitor.next_value());
                Ok(v.value)
            }
        }
        deserializer.deserialize_any(NumberVisitor)
    }
}
