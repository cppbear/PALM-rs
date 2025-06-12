fn unwrap_class_unicode(self) -> hir::ClassUnicode {
        match self {
            HirFrame::ClassUnicode(cls) => cls,
            _ => panic!("tried to unwrap Unicode class \
                         from HirFrame, got: {:?}", self)
        }
    }