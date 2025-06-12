fn serialize_str(self, value: &str) -> Result<()> {
        format_escaped_str(&mut self.writer, &mut self.formatter, value).map_err(Error::io)
    }