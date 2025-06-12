fn flush(&mut self) -> io::Result<()> {
        self.encoder.flush()
    }