fn serialize_i8(self, value: i8) -> Result<Value> {
        self.serialize_i64(value as i64)
    }