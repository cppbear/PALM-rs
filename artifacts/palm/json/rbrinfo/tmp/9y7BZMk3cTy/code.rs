fn serialize_i16(self, value: i16) -> Result<Value> {
        self.serialize_i64(value as i64)
    }