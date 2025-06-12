fn serialize_none(self) -> Result<String> {
        Err(key_must_be_a_string())
    }