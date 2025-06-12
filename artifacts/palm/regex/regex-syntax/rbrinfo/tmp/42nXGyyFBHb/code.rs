pub fn into_kind(mut self) -> HirKind {
        use std::mem;
        mem::replace(&mut self.kind, HirKind::Empty)
    }