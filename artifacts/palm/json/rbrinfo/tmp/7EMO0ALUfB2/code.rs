fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            SerializeMap::Map { map, next_key } => {
                let key = next_key.take();
                // Panic because this indicates a bug in the program rather than an
                // expected failure.
                let key = key.expect("serialize_value called before serialize_key");
                map.insert(key, tri!(to_value(value)));
                Ok(())
            }
            #[cfg(feature = "arbitrary_precision")]
            SerializeMap::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            SerializeMap::RawValue { .. } => unreachable!(),
        }
    }