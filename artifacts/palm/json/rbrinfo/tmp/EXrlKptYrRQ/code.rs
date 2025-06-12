fn serialize_none(self) -> Result<()> {
        Err(key_must_be_a_string())
    }