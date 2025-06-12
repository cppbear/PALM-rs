fn prefixes(lits: &Literals) -> Self {
        let sset = SingleByteSet::prefixes(lits);
        Matcher::new(lits, sset)
    }