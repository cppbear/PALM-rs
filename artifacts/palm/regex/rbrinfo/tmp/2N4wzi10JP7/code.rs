fn c_char(&mut self, c: char) -> Result {
        self.c_class(&[hir::ClassUnicodeRange::new(c, c)])
    }