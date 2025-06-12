fn parse_perl_class(&self) -> ast::ClassPerl {
        let c = self.char();
        let span = self.span_char();
        self.bump();
        let (negated, kind) = match c {
            'd' => (false, ast::ClassPerlKind::Digit),
            'D' => (true, ast::ClassPerlKind::Digit),
            's' => (false, ast::ClassPerlKind::Space),
            'S' => (true, ast::ClassPerlKind::Space),
            'w' => (false, ast::ClassPerlKind::Word),
            'W' => (true, ast::ClassPerlKind::Word),
            c => panic!("expected valid Perl class but got '{}'", c),
        };
        ast::ClassPerl { span: span, kind: kind, negated: negated }
    }