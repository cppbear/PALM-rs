fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Null => visitor.visit_unit(),
            Value::Bool(v) => visitor.visit_bool(v),
            Value::Number(n) => n.deserialize_any(visitor),
            #[cfg(any(feature = "std", feature = "alloc"))]
            Value::String(v) => visitor.visit_string(v),
            #[cfg(not(any(feature = "std", feature = "alloc")))]
            Value::String(_) => unreachable!(),
            Value::Array(v) => visit_array(v, visitor),
            Value::Object(v) => v.deserialize_any(visitor),
        }
    }