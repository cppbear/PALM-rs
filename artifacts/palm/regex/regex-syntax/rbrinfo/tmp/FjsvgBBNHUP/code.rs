pub fn is_negated(&self) -> bool {
        match *self {
            WordBoundary::Unicode |  WordBoundary::Ascii => false,
            WordBoundary::UnicodeNegate |  WordBoundary::AsciiNegate => true,
        }
    }