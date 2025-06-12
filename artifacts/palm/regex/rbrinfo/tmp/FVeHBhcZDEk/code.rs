fn c_byte(&mut self, b: u8) -> Result {
        self.c_class_bytes(&[hir::ClassBytesRange::new(b, b)])
    }