fn unicode_fold_and_negate(
        &self,
        negated: bool,
        class: &mut hir::ClassUnicode,
    ) {
        // Note that we must apply case folding before negation!
        // Consider `(?i)[^x]`. If we applied negation field, then
        // the result would be the character class that matched any
        // Unicode scalar value.
        if self.flags().case_insensitive() {
            class.case_fold_simple();
        }
        if negated {
            class.negate();
        }
    }