fn char_at(&self, i: usize) -> char {
        self.pattern()[i..].chars().next()
            .unwrap_or_else(|| {
                panic!("expected char at offset {}", i)
            })
    }