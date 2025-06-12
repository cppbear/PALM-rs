fn hir_dot(&self, span: Span) -> Result<Hir> {
        let unicode = self.flags().unicode();
        if !unicode && !self.trans().allow_invalid_utf8 {
            return Err(self.error(span, ErrorKind::InvalidUtf8));
        }
        Ok(if self.flags().dot_matches_new_line() {
            Hir::any(!unicode)
        } else {
            Hir::dot(!unicode)
        })
    }