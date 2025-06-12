pub fn new(read: R) -> Self {
        Deserializer {
            read,
            scratch: Vec::new(),
            remaining_depth: 128,
            #[cfg(feature = "float_roundtrip")]
            single_precision: false,
            #[cfg(feature = "unbounded_depth")]
            disable_recursion_limit: false,
        }
    }