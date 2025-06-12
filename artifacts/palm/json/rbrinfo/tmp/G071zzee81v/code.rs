fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Null => visitor.visit_unit(),
            Value::Bool(v) => visitor.visit_bool(*v),
            Value::Number(n) => n.deserialize_any(visitor),
            Value::String(v) => visitor.visit_borrowed_str(v),
            Value::Array(v) => visit_array_ref(v, visitor),
            Value::Object(v) => v.deserialize_any(visitor),
        }
    }