fn hir_from_char_case_insensitive(
        &self,
        span: Span,
        c: char,
    ) -> Result<Hir> {
        // If case folding won't do anything, then don't bother trying.
        if !unicode::contains_simple_case_mapping(c, c) {
            return self.hir_from_char(span, c);
        }
        if self.flags().unicode() {
            let mut cls = hir::ClassUnicode::new(vec![
                hir::ClassUnicodeRange::new(c, c),
            ]);
            cls.case_fold_simple();
            Ok(Hir::class(hir::Class::Unicode(cls)))
        } else {
            if c.len_utf8() > 1 {
                return Err(self.error(span, ErrorKind::UnicodeNotAllowed));
            }
            let mut cls = hir::ClassBytes::new(vec![
                hir::ClassBytesRange::new(c as u8, c as u8),
            ]);
            cls.case_fold_simple();
            Ok(Hir::class(hir::Class::Bytes(cls)))
        }
    }