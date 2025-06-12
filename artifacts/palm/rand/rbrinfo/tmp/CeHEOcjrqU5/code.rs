pub fn reseed(&mut self) -> Result<(), Rsdr::Error> {
        self.0.reset();
        self.0.core.reseed()
    }