fn unwrap_class_bytes(self) -> hir::ClassBytes {
        match self {
            HirFrame::ClassBytes(cls) => cls,
            _ => panic!("tried to unwrap byte class \
                         from HirFrame, got: {:?}", self)
        }
    }