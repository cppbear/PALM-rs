fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use super::SerializeStruct;
        let mut state = tri!(serializer.serialize_struct("Duration", 2));
        tri!(state.serialize_field("secs", &self.as_secs()));
        tri!(state.serialize_field("nanos", &self.subsec_nanos()));
        state.end()
    }