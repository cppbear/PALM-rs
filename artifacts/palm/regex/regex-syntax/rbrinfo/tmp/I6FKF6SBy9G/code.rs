fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match *self {
            ClassFrame::Union{..} => "Union",
            ClassFrame::Binary{..} => "Binary",
            ClassFrame::BinaryLHS{..} => "BinaryLHS",
            ClassFrame::BinaryRHS{..} => "BinaryRHS",
        };
        write!(f, "{}", x)
    }