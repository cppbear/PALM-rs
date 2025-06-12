fn serialize_str(self, value: &str) -> Result<()> {
        self.ser.serialize_str(value)
    }