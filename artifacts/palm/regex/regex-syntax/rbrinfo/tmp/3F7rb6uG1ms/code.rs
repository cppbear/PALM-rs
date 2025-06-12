pub fn is_equal(&self) -> bool {
        match *self {
            ClassUnicodeOpKind::Equal|ClassUnicodeOpKind::Colon => true,
            _ => false,
        }
    }