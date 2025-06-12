pub fn add_char_class(&mut self, cls: &hir::ClassUnicode) -> bool {
        self._add_char_class(cls, false)
    }