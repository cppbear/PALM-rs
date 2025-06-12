fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.next_u64())
    }