fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use super::SerializeStruct;
        let mut state = tri!(serializer.serialize_struct("RangeTo", 1));
        tri!(state.serialize_field("end", &self.end));
        state.end()
    }