pub fn new(bytes: Vec<u8>) -> Literal {
        Literal { v: bytes, cut: false }
    }