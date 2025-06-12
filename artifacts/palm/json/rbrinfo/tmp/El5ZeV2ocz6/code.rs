fn serialize_u32(self, value: u32) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }