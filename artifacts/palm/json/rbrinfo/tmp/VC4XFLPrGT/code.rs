fn serialize_i128(self, value: i128) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }