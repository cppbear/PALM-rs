fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use super::SerializeStruct;
        let duration_since_epoch = match self.duration_since(UNIX_EPOCH) {
            Ok(duration_since_epoch) => duration_since_epoch,
            Err(_) => return Err(S::Error::custom("SystemTime must be later than UNIX_EPOCH")),
        };
        let mut state = tri!(serializer.serialize_struct("SystemTime", 2));
        tri!(state.serialize_field("secs_since_epoch", &duration_since_epoch.as_secs()));
        tri!(state.serialize_field("nanos_since_epoch", &duration_since_epoch.subsec_nanos()));
        state.end()
    }