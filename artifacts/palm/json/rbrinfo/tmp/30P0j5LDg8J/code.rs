fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        match name {
            #[cfg(feature = "arbitrary_precision")]
            crate::number::TOKEN => Ok(Compound::Number { ser: self }),
            #[cfg(feature = "raw_value")]
            crate::raw::TOKEN => Ok(Compound::RawValue { ser: self }),
            _ => self.serialize_map(Some(len)),
        }
    }