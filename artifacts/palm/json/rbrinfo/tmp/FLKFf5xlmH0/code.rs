fn serialize_u8(self, value: u8) -> Result<Value> {
        self.serialize_u64(value as u64)
    }