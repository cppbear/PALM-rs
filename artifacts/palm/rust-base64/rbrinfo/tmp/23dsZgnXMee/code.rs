pub fn encode<S: Sink>(&self, bytes: &[u8], sink: &mut S) -> Result<(), S::Error> {
        const BUF_SIZE: usize = 1024;
        const CHUNK_SIZE: usize = BUF_SIZE / 4 * 3;

        let mut buf = [0; BUF_SIZE];
        for chunk in bytes.chunks(CHUNK_SIZE) {
            let mut len = self.engine.internal_encode(chunk, &mut buf);
            if chunk.len() != CHUNK_SIZE && self.engine.config().encode_padding() {
                // Final, potentially partial, chunk.
                // Only need to consider if padding is needed on a partial chunk since full chunk
                // is a multiple of 3, which therefore won't be padded.
                // Pad output to multiple of four bytes if required by config.
                len += add_padding(len, &mut buf[len..]);
            }
            sink.write_encoded_bytes(&buf[..len])?;
        }

        Ok(())
    }