fn next_capture_index(&self, span: Span) -> Result<u32> {
        let current = self.parser().capture_index.get();
        let i = current.checked_add(1).ok_or_else(|| {
            self.error(span, ast::ErrorKind::CaptureLimitExceeded)
        })?;
        self.parser().capture_index.set(i);
        Ok(i)
    }