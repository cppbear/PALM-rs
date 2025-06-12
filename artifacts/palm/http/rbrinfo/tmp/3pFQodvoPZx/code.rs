fn parse_non_empty(s: &[u8]) -> Result<usize, InvalidUri> {
        if s.is_empty() {
            return Err(ErrorKind::Empty.into());
        }
        Authority::parse(s)
    }