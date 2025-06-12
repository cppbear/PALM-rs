fn get_i8(&mut self) -> i8 {
        if self.remaining() < 1 {
            panic_advance(&TryGetError {
                requested: 1,
                available: 0,
            });
        }
        let ret = self.chunk()[0] as i8;
        self.advance(1);
        ret
    }