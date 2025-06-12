fn read_from_delegate(&mut self) -> io::Result<usize> {
        debug_assert!(self.b64_offset + self.b64_len < BUF_SIZE);

        let read = self
            .inner
            .read(&mut self.b64_buffer[self.b64_offset + self.b64_len..])?;
        self.b64_len += read;

        debug_assert!(self.b64_offset + self.b64_len <= BUF_SIZE);

        Ok(read)
    }