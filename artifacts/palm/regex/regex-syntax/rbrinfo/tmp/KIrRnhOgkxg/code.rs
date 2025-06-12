fn maybe_parse_ascii_class(&self) -> Option<ast::ClassAscii> {
        // ASCII character classes are interesting from a parsing perspective
        // because parsing cannot fail with any interesting error. For example,
        // in order to use an ASCII character class, it must be enclosed in
        // double brackets, e.g., `[[:alnum:]]`. Alternatively, you might think
        // of it as "ASCII character characters have the syntax `[:NAME:]`
        // which can only appear within character brackets." This means that
        // things like `[[:lower:]A]` are legal constructs.
        //
        // However, if one types an incorrect ASCII character class, e.g.,
        // `[[:loower:]]`, then we treat that as a normal nested character
        // class containing the characters `:elorw`. One might argue that we
        // should return an error instead since the repeated colons give away
        // the intent to write an ASCII class. But what if the user typed
        // `[[:lower]]` instead? How can we tell that was intended to be an
        // ASCII class and not just a normal nested class?
        //
        // Reasonable people can probably disagree over this, but for better
        // or worse, we implement semantics that never fails at the expense
        // of better failure modes.
        assert_eq!(self.char(), '[');
        // If parsing fails, then we back up the parser to this starting point.
        let start = self.pos();
        let mut negated = false;
        if !self.bump() || self.char() != ':' {
            self.parser().pos.set(start);
            return None;
        }
        if !self.bump() {
            self.parser().pos.set(start);
            return None;
        }
        if self.char() == '^' {
            negated = true;
            if !self.bump() {
                self.parser().pos.set(start);
                return None;
            }
        }
        let name_start = self.offset();
        while self.char() != ':' && self.bump() {}
        if self.is_eof() {
            self.parser().pos.set(start);
            return None;
        }
        let name = &self.pattern()[name_start..self.offset()];
        if !self.bump_if(":]") {
            self.parser().pos.set(start);
            return None;
        }
        let kind = match ast::ClassAsciiKind::from_name(name) {
            Some(kind) => kind,
            None => {
                self.parser().pos.set(start);
                return None;
            }
        };
        Some(ast::ClassAscii {
            span: Span::new(start, self.pos()),
            kind: kind,
            negated: negated,
        })
    }