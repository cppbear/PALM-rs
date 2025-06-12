pub fn from_slice(bytes: &'a [u8]) -> Self {
        Deserializer::new(read::SliceRead::new(bytes))
    }