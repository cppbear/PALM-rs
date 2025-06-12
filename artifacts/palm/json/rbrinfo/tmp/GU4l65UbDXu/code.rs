fn serialize_u8(self, value: u8) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }