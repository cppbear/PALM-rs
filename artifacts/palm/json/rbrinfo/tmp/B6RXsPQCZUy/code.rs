fn serialize_u128(self, value: u128) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }