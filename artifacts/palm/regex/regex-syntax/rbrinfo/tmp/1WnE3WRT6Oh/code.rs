fn parse_decimal(&self) -> Result<u32> {
        let mut scratch = self.parser().scratch.borrow_mut();
        scratch.clear();

        while !self.is_eof() && self.char().is_whitespace() {
            self.bump();
        }
        let start = self.pos();
        while !self.is_eof() && '0' <= self.char() && self.char() <= '9' {
            scratch.push(self.char());
            self.bump_and_bump_space();
        }
        let span = Span::new(start, self.pos());
        while !self.is_eof() && self.char().is_whitespace() {
            self.bump_and_bump_space();
        }
        let digits = scratch.as_str();
        if digits.is_empty() {
            return Err(self.error(span, ast::ErrorKind::DecimalEmpty));
        }
        match u32::from_str_radix(digits, 10).ok() {
            Some(n) => Ok(n),
            None => Err(self.error(span, ast::ErrorKind::DecimalInvalid)),
        }
    }