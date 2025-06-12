fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            const MAX_LEN: usize = 58;
            debug_assert_eq!(
                MAX_LEN,
                "[1001:1002:1003:1004:1005:1006:1007:1008%4294967295]:65000".len()
            );
            serialize_display_bounded_length!(self, MAX_LEN, serializer)
        } else {
            (self.ip(), self.port()).serialize(serializer)
        }
    }