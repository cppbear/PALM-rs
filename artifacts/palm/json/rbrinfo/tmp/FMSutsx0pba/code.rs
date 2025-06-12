fn end(self) -> Result<()> {
        match self {
            Compound::Map { .. } => ser::SerializeMap::end(self),
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => Ok(()),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => Ok(()),
        }
    }