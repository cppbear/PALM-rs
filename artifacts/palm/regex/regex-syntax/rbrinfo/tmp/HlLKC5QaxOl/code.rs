fn increment(self) -> Self {
        match self {
            '\u{D7FF}' => '\u{E000}',
            c => char::from_u32((c as u32).checked_add(1).unwrap()).unwrap(),
        }
    }