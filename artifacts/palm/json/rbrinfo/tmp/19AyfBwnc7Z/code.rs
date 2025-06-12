fn serialize_i16(self, value: i16) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }