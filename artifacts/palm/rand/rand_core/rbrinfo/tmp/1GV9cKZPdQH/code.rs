fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.next_u32())
    }