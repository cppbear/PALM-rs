fn unwrap_group(self) -> Option<Flags> {
        match self {
            HirFrame::Group { old_flags } => old_flags,
            _ => panic!("tried to unwrap group from HirFrame, got: {:?}", self)
        }
    }