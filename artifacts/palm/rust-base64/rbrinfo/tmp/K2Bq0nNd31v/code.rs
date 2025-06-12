pub fn from_consumer(str_consumer: S, engine: &'e E) -> Self {
        EncoderStringWriter {
            encoder: EncoderWriter::new(Utf8SingleCodeUnitWriter { str_consumer }, engine),
        }
    }