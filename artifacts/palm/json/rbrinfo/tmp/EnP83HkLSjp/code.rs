fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            SerializeMap::Map { next_key, .. } => {
                *next_key = Some(tri!(key.serialize(MapKeySerializer)));
                Ok(())
            }
            #[cfg(feature = "arbitrary_precision")]
            SerializeMap::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            SerializeMap::RawValue { .. } => unreachable!(),
        }
    }