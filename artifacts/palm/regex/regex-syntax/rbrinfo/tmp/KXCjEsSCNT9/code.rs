pub fn is_unicode(&self) -> bool {
        match *self {
            Literal::Unicode(_) => true,
            Literal::Byte(b) if b <= 0x7F => true,
            Literal::Byte(_) => false,
        }
    }