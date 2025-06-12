fn try_get_i8(&mut self) -> Result<i8, TryGetError> {
        if self.remaining() < 1 {
            return Err(TryGetError {
                requested: 1,
                available: self.remaining(),
            });
        }
        let ret = self.chunk()[0] as i8;
        self.advance(1);
        Ok(ret)
    }