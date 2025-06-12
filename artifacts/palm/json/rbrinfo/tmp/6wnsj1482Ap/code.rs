fn serialize_u128(self, value: u128) -> Result<()> {
        self.formatter
            .write_u128(&mut self.writer, value)
            .map_err(Error::io)
    }