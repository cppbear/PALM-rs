fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Follow `vec::Splice` in only printing the drain and replacement
        f.debug_struct("Splice")
            .field("drain", &self.drain)
            .field("replace_with", &self.replace_with)
            .finish()
    }