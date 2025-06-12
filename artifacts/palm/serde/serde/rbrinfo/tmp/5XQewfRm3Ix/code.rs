fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            const MAX_LEN: usize = 21;
            debug_assert_eq!(MAX_LEN, "101.102.103.104:65000".len());
            serialize_display_bounded_length!(self, MAX_LEN, serializer)
        } else {
            (self.ip(), self.port()).serialize(serializer)
        }
    }