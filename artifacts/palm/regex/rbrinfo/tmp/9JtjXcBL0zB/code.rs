fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, [u8]>> {
        self.0.no_expansion()
    }