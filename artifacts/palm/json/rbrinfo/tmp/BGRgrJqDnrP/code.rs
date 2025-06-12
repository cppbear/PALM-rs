fn serialize_i64(self, value: i64) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }