pub fn from_reader(reader: R) -> Self {
        Deserializer::new(read::IoRead::new(reader))
    }