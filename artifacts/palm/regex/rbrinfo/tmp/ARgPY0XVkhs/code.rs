fn new(lits: &Literals, sset: SingleByteSet) -> Self {
        if lits.literals().is_empty() {
            return Matcher::Empty;
        }
        if sset.dense.len() >= 26 {
            // Avoid trying to match a large number of single bytes.
            // This is *very* sensitive to a frequency analysis comparison
            // between the bytes in sset and the composition of the haystack.
            // No matter the size of sset, if its members all are rare in the
            // haystack, then it'd be worth using it. How to tune this... IDK.
            // ---AG
            return Matcher::Empty;
        }
        if sset.complete {
            return Matcher::Bytes(sset);
        }
        if lits.literals().len() == 1 {
            let lit = lits.literals()[0].to_vec();
            if BoyerMooreSearch::should_use(lit.as_slice()) {
                return Matcher::BoyerMoore(BoyerMooreSearch::new(lit));
            } else {
                return Matcher::FreqyPacked(FreqyPacked::new(lit));
            }
        }
        let is_aho_corasick_fast = sset.dense.len() == 1 && sset.all_ascii;
        if TeddyAVX2::available() && !is_aho_corasick_fast {
            const MAX_TEDDY_LITERALS: usize = 32;
            if lits.literals().len() <= MAX_TEDDY_LITERALS {
                if let Some(ted) = TeddyAVX2::new(lits) {
                    return Matcher::TeddyAVX2(ted);
                }
            }
        }
        if TeddySSSE3::available() && !is_aho_corasick_fast {
            // Only try Teddy if Aho-Corasick can't use memchr on an ASCII
            // byte. Also, in its current form, Teddy doesn't scale well to
            // lots of literals.
            //
            // We impose the ASCII restriction since an alternation of
            // non-ASCII string literals in the same language is likely to all
            // start with the same byte. Even worse, the corpus being searched
            // probably has a similar composition, which ends up completely
            // negating the benefit of memchr.
            const MAX_TEDDY_LITERALS: usize = 32;
            if lits.literals().len() <= MAX_TEDDY_LITERALS {
                if let Some(ted) = TeddySSSE3::new(lits) {
                    return Matcher::TeddySSSE3(ted);
                }
            }
            // Fallthrough to ol' reliable Aho-Corasick...
        }
        let pats = lits.literals().to_owned();
        Matcher::AC(AcAutomaton::new(pats).into_full())
    }