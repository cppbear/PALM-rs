fn ignore_decimal(&mut self) -> Result<()> {
        self.eat_char();

        let mut at_least_one_digit = false;
        while let b'0'..=b'9' = tri!(self.peek_or_null()) {
            self.eat_char();
            at_least_one_digit = true;
        }

        if !at_least_one_digit {
            return Err(self.peek_error(ErrorCode::InvalidNumber));
        }

        match tri!(self.peek_or_null()) {
            b'e' | b'E' => self.ignore_exponent(),
            _ => Ok(()),
        }
    }