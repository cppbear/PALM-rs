fn suffixes(lits: &Literals) -> Self {
        let sset = SingleByteSet::suffixes(lits);
        Matcher::new(lits, sset)
    }