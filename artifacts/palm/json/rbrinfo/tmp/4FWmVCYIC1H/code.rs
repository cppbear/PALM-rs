fn serialize_u16(self, value: u16) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }