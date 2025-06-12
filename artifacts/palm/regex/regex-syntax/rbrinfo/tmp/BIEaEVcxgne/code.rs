pub fn is_empty(&self) -> bool {
        match *self {
            HirKind::Empty => true,
            _ => false,
        }
    }