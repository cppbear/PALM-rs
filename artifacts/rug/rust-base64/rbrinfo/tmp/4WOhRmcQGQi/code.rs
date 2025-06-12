pub fn new(delegate: W, engine: &'e E) -> EncoderWriter<'e, E, W> {
        EncoderWriter {
            engine,
            delegate: Some(delegate),
            extra_input: [0u8; MIN_ENCODE_CHUNK_SIZE],
            extra_input_occupied_len: 0,
            output: [0u8; BUF_SIZE],
            output_occupied_len: 0,
            panicked: false,
        }
    }