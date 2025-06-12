fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Because we expect all input to be valid utf-8 individual bytes, we can encode any buffer
        // length
        let s = std::str::from_utf8(buf).expect("Input must be valid UTF-8");

        self.str_consumer.consume(s);

        Ok(buf.len())
    }