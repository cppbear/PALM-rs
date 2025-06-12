fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Ok(self.buf.chunk())
    }