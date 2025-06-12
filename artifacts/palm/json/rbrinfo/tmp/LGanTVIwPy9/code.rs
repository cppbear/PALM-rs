fn end(self) -> Result<Value> {
        match self {
            SerializeMap::Map { .. } => serde::ser::SerializeMap::end(self),
            #[cfg(feature = "arbitrary_precision")]
            SerializeMap::Number { out_value, .. } => {
                Ok(out_value.expect("number value was not emitted"))
            }
            #[cfg(feature = "raw_value")]
            SerializeMap::RawValue { out_value, .. } => {
                Ok(out_value.expect("raw value was not emitted"))
            }
        }
    }