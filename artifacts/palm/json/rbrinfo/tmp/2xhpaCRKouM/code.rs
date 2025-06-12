fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Compound::Map { .. } => ser::SerializeMap::serialize_entry(self, key, value),
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { ser, .. } => {
                if key == crate::number::TOKEN {
                    value.serialize(NumberStrEmitter(ser))
                } else {
                    Err(invalid_number())
                }
            }
            #[cfg(feature = "raw_value")]
            Compound::RawValue { ser, .. } => {
                if key == crate::raw::TOKEN {
                    value.serialize(RawValueStrEmitter(ser))
                } else {
                    Err(invalid_raw_value())
                }
            }
        }
    }