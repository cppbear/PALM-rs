pub fn is_empty(&self) -> bool {
        match *self {
            Ast::Empty(_) => true,
            _ => false,
        }
    }