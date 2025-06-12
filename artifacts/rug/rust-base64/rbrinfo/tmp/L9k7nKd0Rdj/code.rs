pub fn new(engine: &'e E) -> ChunkedEncoder<'e, E> {
        ChunkedEncoder { engine }
    }