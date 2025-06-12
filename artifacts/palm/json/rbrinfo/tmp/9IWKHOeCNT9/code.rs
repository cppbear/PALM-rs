fn serialize_none(self) -> Result<Value> {
        self.serialize_unit()
    }