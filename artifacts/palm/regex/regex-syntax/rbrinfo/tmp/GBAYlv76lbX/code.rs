fn decrement(self) -> Self {
        match self {
            '\u{E000}' => '\u{D7FF}',
            c => char::from_u32((c as u32).checked_sub(1).unwrap()).unwrap(),
        }
    }