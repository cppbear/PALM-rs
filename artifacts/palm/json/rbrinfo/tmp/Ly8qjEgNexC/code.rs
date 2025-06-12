fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        self.formatter
            .write_byte_array(&mut self.writer, value)
            .map_err(Error::io)
    }