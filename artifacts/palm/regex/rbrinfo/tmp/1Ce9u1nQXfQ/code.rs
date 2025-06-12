fn prefixes(lits: &Literals) -> SingleByteSet {
        let mut sset = SingleByteSet::new();
        for lit in lits.literals() {
            sset.complete = sset.complete && lit.len() == 1;
            if let Some(&b) = lit.get(0) {
                if !sset.sparse[b as usize] {
                    if b > 0x7F {
                        sset.all_ascii = false;
                    }
                    sset.dense.push(b);
                    sset.sparse[b as usize] = true;
                }
            }
        }
        sset
    }