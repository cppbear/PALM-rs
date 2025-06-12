fn end(self) -> Result<Value> {
        match self {
            SerializeMap::Map { map, .. } => Ok(Value::Object(map)),
            #[cfg(feature = "arbitrary_precision")]
            SerializeMap::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            SerializeMap::RawValue { .. } => unreachable!(),
        }
    }