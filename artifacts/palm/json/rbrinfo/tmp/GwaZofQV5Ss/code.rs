fn serialize_f64(self, value: f64) -> Result<()> {
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
            .write_f64(&mut self.ser.writer, value)
            .map_err(Error::io));
        self.ser
            .formatter
            .end_string(&mut self.ser.writer)
            .map_err(Error::io)
    }