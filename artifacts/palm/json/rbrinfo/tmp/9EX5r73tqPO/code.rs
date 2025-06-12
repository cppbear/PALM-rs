fn serialize_u16(self, value: u16) -> Result<Value> {
        self.serialize_u64(value as u64)
    }