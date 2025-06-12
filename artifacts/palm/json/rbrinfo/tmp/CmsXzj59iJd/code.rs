fn serialize_i32(self, value: i32) -> Result<Value> {
        self.serialize_i64(value as i64)
    }