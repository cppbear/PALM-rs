fn serialize_f32(self, value: f32) -> Result<()> {
        if !value.is_finite() {
            return Err(float_key_must_be_finite());
        }

        tri!(self
            .ser
            .formatter
            .begin_string(&mut self.ser.writer)
            .map_err(Error::io));
        tri!(self
            .ser
            .formatter
            .write_f32(&mut self.ser.writer, value)
            .map_err(Error::io));
        self.ser
            .formatter
            .end_string(&mut self.ser.writer)
            .map_err(Error::io)
    }