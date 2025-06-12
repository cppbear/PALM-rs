fn serialize_char(self, value: char) -> Result<()> {
        self.ser.serialize_str(value.encode_utf8(&mut [0u8; 4]))
    }