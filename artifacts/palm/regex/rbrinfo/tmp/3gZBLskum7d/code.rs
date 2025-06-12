fn new(haystack: &'t str, start: usize, end: usize) -> Match<'t> {
        Match {
            text: haystack,
            start: start,
            end: end,
        }
    }