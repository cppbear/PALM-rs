fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DecoderReader")
            .field("b64_offset", &self.b64_offset)
            .field("b64_len", &self.b64_len)
            .field("decoded_chunk_buffer", &self.decoded_chunk_buffer)
            .field("decoded_offset", &self.decoded_offset)
            .field("decoded_len", &self.decoded_len)
            .field("input_consumed_len", &self.input_consumed_len)
            .field("padding_offset", &self.padding_offset)
            .finish()
    }