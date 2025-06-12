pub fn is_negation(&self) -> bool {
        match *self {
            FlagsItemKind::Negation => true,
            _ => false,
        }
    }