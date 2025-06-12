fn serialize_i32(self, value: i32) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }