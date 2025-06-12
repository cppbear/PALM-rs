fn serialize_bool(self, value: bool) -> Result<String> {
        Ok(if value { "true" } else { "false" }.to_owned())
    }