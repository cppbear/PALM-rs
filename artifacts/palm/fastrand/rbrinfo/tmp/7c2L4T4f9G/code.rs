pub fn bool(&mut self) -> bool {
        self.u8(..) % 2 == 0
    }