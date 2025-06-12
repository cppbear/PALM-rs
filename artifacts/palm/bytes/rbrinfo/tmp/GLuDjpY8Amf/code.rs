fn get_u8(&mut self) -> u8 {
        if self.remaining() < 1 {
            panic_advance(&TryGetError {
                requested: 1,
                available: 0,
            })
        }
        let ret = self.chunk()[0];
        self.advance(1);
        ret
    }