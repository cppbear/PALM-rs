fn new(haystack: &'t [u8], start: usize, end: usize) -> Match<'t> {
        Match {
            text: haystack,
            start: start,
            end: end,
        }
    }