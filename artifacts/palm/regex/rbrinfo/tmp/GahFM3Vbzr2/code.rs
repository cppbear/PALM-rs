fn parse(&self) -> Result<Parsed, Error> {
        let mut exprs = Vec::with_capacity(self.options.pats.len());
        let mut prefixes = Some(Literals::empty());
        let mut suffixes = Some(Literals::empty());
        let mut bytes = false;
        let is_set = self.options.pats.len() > 1;
        // If we're compiling a regex set and that set has any anchored
        // expressions, then disable all literal optimizations.
        for pat in &self.options.pats {
            let mut parser =
                ParserBuilder::new()
                    .octal(self.options.octal)
                    .case_insensitive(self.options.case_insensitive)
                    .multi_line(self.options.multi_line)
                    .dot_matches_new_line(self.options.dot_matches_new_line)
                    .swap_greed(self.options.swap_greed)
                    .ignore_whitespace(self.options.ignore_whitespace)
                    .unicode(self.options.unicode)
                    .allow_invalid_utf8(!self.only_utf8)
                    .nest_limit(self.options.nest_limit)
                    .build();
            let expr = parser
                .parse(pat)
                .map_err(|e| Error::Syntax(e.to_string()))?;
            bytes = bytes || !expr.is_always_utf8();

            if !expr.is_anchored_start() && expr.is_any_anchored_start() {
                // Partial anchors unfortunately make it hard to use prefixes,
                // so disable them.
                prefixes = None;
            } else if is_set && expr.is_anchored_start() {
                // Regex sets with anchors do not go well with literal
                // optimizations.
                prefixes = None;
            }
            prefixes = prefixes.and_then(|mut prefixes| {
                if !prefixes.union_prefixes(&expr) {
                    None
                } else {
                    Some(prefixes)
                }
            });

            if !expr.is_anchored_end() && expr.is_any_anchored_end() {
                // Partial anchors unfortunately make it hard to use suffixes,
                // so disable them.
                suffixes = None;
            } else if is_set && expr.is_anchored_end() {
                // Regex sets with anchors do not go well with literal
                // optimizations.
                suffixes = None;
            }
            suffixes = suffixes.and_then(|mut suffixes| {
                if !suffixes.union_suffixes(&expr) {
                    None
                } else {
                    Some(suffixes)
                }
            });
            exprs.push(expr);
        }
        Ok(Parsed {
            exprs: exprs,
            prefixes: prefixes.unwrap_or_else(Literals::empty),
            suffixes: suffixes.unwrap_or_else(Literals::empty),
            bytes: bytes,
        })
    }