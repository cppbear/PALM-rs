fn increment_depth(&mut self, span: &Span) -> Result<()> {
        let new = self.depth.checked_add(1).ok_or_else(|| self.p.error(
            span.clone(),
            ast::ErrorKind::NestLimitExceeded(::std::u32::MAX),
        ))?;
        let limit = self.p.parser().nest_limit;
        if new > limit {
            return Err(self.p.error(
                span.clone(),
                ast::ErrorKind::NestLimitExceeded(limit),
            ));
        }
        self.depth = new;
        Ok(())
    }