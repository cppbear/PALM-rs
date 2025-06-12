fn write_byte_array<W>(&mut self, writer: &mut W, value: &[u8]) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        tri!(self.begin_array(writer));
        let mut first = true;
        for byte in value {
            tri!(self.begin_array_value(writer, first));
            tri!(self.write_u8(writer, *byte));
            tri!(self.end_array_value(writer));
            first = false;
        }
        self.end_array(writer)
    }