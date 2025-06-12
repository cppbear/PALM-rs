fn serialize_u32(self, value: u32) -> Result<()> {
        tri!(self
            .ser
            .formatter
            .begin_string(&mut self.ser.writer)
            .map_err(Error::io));
        tri!(self
            .ser
            .formatter
            .write_u32(&mut self.ser.writer, value)
            .map_err(Error::io));
        self.ser
            .formatter
            .end_string(&mut self.ser.writer)
            .map_err(Error::io)
    }