pub fn new(state: u128) -> Self {
        // Force low bit to 1, as in C version (C++ uses `state | 3` instead).
        Mcg128Xsl64 { state: state | 1 }
    }