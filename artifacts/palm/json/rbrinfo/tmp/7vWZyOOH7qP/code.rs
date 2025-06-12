fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            SerializeMap::Map { .. } => serde::ser::SerializeMap::serialize_entry(self, key, value),
            #[cfg(feature = "arbitrary_precision")]
            SerializeMap::Number { out_value } => {
                if key == crate::number::TOKEN {
                    *out_value = Some(tri!(value.serialize(NumberValueEmitter)));
                    Ok(())
                } else {
                    Err(invalid_number())
                }
            }
            #[cfg(feature = "raw_value")]
            SerializeMap::RawValue { out_value } => {
                if key == crate::raw::TOKEN {
                    *out_value = Some(tri!(value.serialize(RawValueEmitter)));
                    Ok(())
                } else {
                    Err(invalid_raw_value())
                }
            }
        }
    }