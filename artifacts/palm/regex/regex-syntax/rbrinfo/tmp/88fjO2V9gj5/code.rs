pub fn build(&self) -> Translator {
        Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(self.flags),
            allow_invalid_utf8: self.allow_invalid_utf8,
        }
    }