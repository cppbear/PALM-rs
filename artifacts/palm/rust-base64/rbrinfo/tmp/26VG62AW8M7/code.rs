pub fn new(bytes: &'a [u8], engine: &'e E) -> Base64Display<'a, 'e, E> {
        Base64Display {
            bytes,
            chunked_encoder: ChunkedEncoder::new(engine),
        }
    }