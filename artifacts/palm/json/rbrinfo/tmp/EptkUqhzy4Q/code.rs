fn serialize_i8(self, value: i8) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }