fn hir_perl_unicode_class(
        &self,
        ast_class: &ast::ClassPerl,
    ) -> hir::ClassUnicode {
        use ast::ClassPerlKind::*;
        use unicode_tables::perl_word::PERL_WORD;

        assert!(self.flags().unicode());
        let mut class = match ast_class.kind {
            Digit => {
                let query = ClassQuery::Binary("Decimal_Number");
                unicode::class(query).unwrap()
            }
            Space => {
                let query = ClassQuery::Binary("Whitespace");
                unicode::class(query).unwrap()
            }
            Word => unicode::hir_class(PERL_WORD),
        };
        // We needn't apply case folding here because the Perl Unicode classes
        // are already closed under Unicode simple case folding.
        if ast_class.negated {
            class.negate();
        }
        class
    }