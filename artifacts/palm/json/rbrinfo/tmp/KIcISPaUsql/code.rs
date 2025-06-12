fn serialize_u64(self, value: u64) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }