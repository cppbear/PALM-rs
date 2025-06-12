fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => formatter.write_str("Null"),
            Value::Bool(boolean) => write!(formatter, "Bool({})", boolean),
            Value::Number(number) => Debug::fmt(number, formatter),
            Value::String(string) => write!(formatter, "String({:?})", string),
            Value::Array(vec) => {
                tri!(formatter.write_str("Array "));
                Debug::fmt(vec, formatter)
            }
            Value::Object(map) => {
                tri!(formatter.write_str("Object "));
                Debug::fmt(map, formatter)
            }
        }
    }