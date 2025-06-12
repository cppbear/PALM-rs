pub fn approximate_size(&self) -> usize {
        // The only instruction that uses heap space is Ranges (for
        // Unicode codepoint programs) to store non-overlapping codepoint
        // ranges. To keep this operation constant time, we ignore them.
        (self.len() * mem::size_of::<Inst>())
        + (self.matches.len() * mem::size_of::<InstPtr>())
        + (self.captures.len() * mem::size_of::<Option<String>>())
        + (self.capture_name_idx.len() *
           (mem::size_of::<String>() + mem::size_of::<usize>()))
        + (self.byte_classes.len() * mem::size_of::<u8>())
        + self.prefixes.approximate_size()
    }