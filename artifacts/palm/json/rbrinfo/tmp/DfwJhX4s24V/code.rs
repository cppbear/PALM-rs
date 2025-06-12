fn serialize_u32(self, value: u32) -> Result<Value> {
        self.serialize_u64(value as u64)
    }