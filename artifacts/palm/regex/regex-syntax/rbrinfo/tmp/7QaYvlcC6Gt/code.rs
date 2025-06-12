fn hir_perl_byte_class(
        &self,
        ast_class: &ast::ClassPerl,
    ) -> hir::ClassBytes {
        use ast::ClassPerlKind::*;

        assert!(!self.flags().unicode());
        let mut class = match ast_class.kind {
            Digit => hir_ascii_class_bytes(&ast::ClassAsciiKind::Digit),
            Space => hir_ascii_class_bytes(&ast::ClassAsciiKind::Space),
            Word => hir_ascii_class_bytes(&ast::ClassAsciiKind::Word),
        };
        // We needn't apply case folding here because the Perl ASCII classes
        // are already closed (under ASCII case folding).
        if ast_class.negated {
            class.negate();
        }
        class
    }