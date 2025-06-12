use super::Value;
use alloc::string::String;
macro_rules! partialeq_numeric {
    ($($eq:ident [$($ty:ty)*])*) => {
        $($(impl PartialEq <$ty > for Value { fn eq(& self, other : &$ty) -> bool { $eq
        (self, * other as _) } } impl PartialEq < Value > for $ty { fn eq(& self, other :
        & Value) -> bool { $eq (other, * self as _) } } impl <'a > PartialEq <$ty > for
        &'a Value { fn eq(& self, other : &$ty) -> bool { $eq (* self, * other as _) } }
        impl <'a > PartialEq <$ty > for &'a mut Value { fn eq(& self, other : &$ty) ->
        bool { $eq (* self, * other as _) } })*)*
    };
}
partialeq_numeric! {
    eq_i64[i8 i16 i32 i64 isize] eq_u64[u8 u16 u32 u64 usize] eq_f32[f32] eq_f64[f64]
    eq_bool[bool]
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Number {
    n: N,
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Value {
    /// Represents a JSON null value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(null);
    /// ```
    Null,
    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(true);
    /// ```
    Bool(bool),
    /// Represents a JSON number, whether integer or floating point.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(12.5);
    /// ```
    Number(Number),
    /// Represents a JSON string.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!("a string");
    /// ```
    String(String),
    /// Represents a JSON array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(["an", "array"]);
    /// ```
    Array(Vec<Value>),
    /// Represents a JSON object.
    ///
    /// By default the map is backed by a BTreeMap. Enable the `preserve_order`
    /// feature of serde_json to use IndexMap instead, which preserves
    /// entries in the order they are inserted into the map. In particular, this
    /// allows JSON data to be deserialized into a Value and serialized to a
    /// string while retaining the order of map keys in the input.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "an": "object" });
    /// ```
    Object(Map<String, Value>),
}
impl Number {
    pub fn is_i64(&self) -> bool {}
    pub fn is_u64(&self) -> bool {}
    pub fn is_f64(&self) -> bool {}
    pub fn as_i64(&self) -> Option<i64> {}
    pub fn as_u64(&self) -> Option<u64> {}
    pub fn as_f64(&self) -> Option<f64> {}
    pub fn from_f64(f: f64) -> Option<Number> {}
    pub fn as_i128(&self) -> Option<i128> {}
    pub fn as_u128(&self) -> Option<u128> {}
    pub fn from_i128(i: i128) -> Option<Number> {}
    pub fn from_u128(i: u128) -> Option<Number> {}
    #[cfg(feature = "arbitrary_precision")]
    #[cfg_attr(docsrs, doc(cfg(feature = "arbitrary_precision")))]
    pub fn as_str(&self) -> &str {}
    pub(crate) fn as_f32(&self) -> Option<f32> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => Some(n as f32),
            N::NegInt(n) => Some(n as f32),
            N::Float(n) => Some(n as f32),
        }
        #[cfg(feature = "arbitrary_precision")]
        self.n.parse::<f32>().ok().filter(|float| float.is_finite())
    }
    pub(crate) fn from_f32(f: f32) -> Option<Number> {}
    #[cfg(feature = "arbitrary_precision")]
    #[inline]
    pub fn from_string_unchecked(n: String) -> Self {
        Number { n }
    }
}
fn eq_f32(value: &Value, other: f32) -> bool {
    match value {
        Value::Number(n) => n.as_f32() == Some(other),
        _ => false,
    }
}
