fn serialize_unit_struct(self, _name: &'static str) -> Result<String> {
        Err(key_must_be_a_string())
    }