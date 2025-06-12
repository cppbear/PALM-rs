pub fn new(reader: R, engine: &'e E) -> Self {
        DecoderReader {
            engine,
            inner: reader,
            b64_buffer: [0; BUF_SIZE],
            b64_offset: 0,
            b64_len: 0,
            decoded_chunk_buffer: [0; DECODED_CHUNK_SIZE],
            decoded_offset: 0,
            decoded_len: 0,
            input_consumed_len: 0,
            padding_offset: None,
        }
    }