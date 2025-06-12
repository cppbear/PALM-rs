pub fn new(engine: &'e E) -> Self {
        EncoderStringWriter::from_consumer(String::new(), engine)
    }