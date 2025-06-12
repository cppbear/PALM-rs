pub fn is_valid(&self) -> bool {
        self.start.c <= self.end.c
    }