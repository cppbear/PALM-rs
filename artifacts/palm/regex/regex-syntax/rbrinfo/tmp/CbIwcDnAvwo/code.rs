fn hir_assertion(&self, asst: &ast::Assertion) -> Result<Hir> {
        let unicode = self.flags().unicode();
        let multi_line = self.flags().multi_line();
        Ok(match asst.kind {
            ast::AssertionKind::StartLine => {
                Hir::anchor(if multi_line {
                    hir::Anchor::StartLine
                } else {
                    hir::Anchor::StartText
                })
            }
            ast::AssertionKind::EndLine => {
                Hir::anchor(if multi_line {
                    hir::Anchor::EndLine
                } else {
                    hir::Anchor::EndText
                })
            }
            ast::AssertionKind::StartText => {
                Hir::anchor(hir::Anchor::StartText)
            }
            ast::AssertionKind::EndText => {
                Hir::anchor(hir::Anchor::EndText)
            }
            ast::AssertionKind::WordBoundary => {
                Hir::word_boundary(if unicode {
                    hir::WordBoundary::Unicode
                } else {
                    hir::WordBoundary::Ascii
                })
            }
            ast::AssertionKind::NotWordBoundary => {
                Hir::word_boundary(if unicode {
                    hir::WordBoundary::UnicodeNegate
                } else {
                    // It is possible for negated ASCII word boundaries to
                    // match at invalid UTF-8 boundaries, even when searching
                    // valid UTF-8.
                    if !self.trans().allow_invalid_utf8 {
                        return Err(self.error(
                            asst.span, ErrorKind::InvalidUtf8));
                    }
                    hir::WordBoundary::AsciiNegate
                })
            }
        })
    }