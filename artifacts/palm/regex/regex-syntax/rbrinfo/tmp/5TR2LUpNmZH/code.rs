fn add_char_class_reverse(&mut self, cls: &hir::ClassUnicode) -> bool {
        self._add_char_class(cls, true)
    }