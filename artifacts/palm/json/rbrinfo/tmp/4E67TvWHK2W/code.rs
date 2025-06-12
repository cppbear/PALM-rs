fn parse_decimal(
        &mut self,
        positive: bool,
        mut significand: u64,
        exponent_before_decimal_point: i32,
    ) -> Result<f64> {
        self.eat_char();

        let mut exponent_after_decimal_point = 0;
        while let c @ b'0'..=b'9' = tri!(self.peek_or_null()) {
            let digit = (c - b'0') as u64;

            if overflow!(significand * 10 + digit, u64::MAX) {
                let exponent = exponent_before_decimal_point + exponent_after_decimal_point;
                return self.parse_decimal_overflow(positive, significand, exponent);
            }

            self.eat_char();
            significand = significand * 10 + digit;
            exponent_after_decimal_point -= 1;
        }

        // Error if there is not at least one digit after the decimal point.
        if exponent_after_decimal_point == 0 {
            match tri!(self.peek()) {
                Some(_) => return Err(self.peek_error(ErrorCode::InvalidNumber)),
                None => return Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
            }
        }

        let exponent = exponent_before_decimal_point + exponent_after_decimal_point;
        match tri!(self.peek_or_null()) {
            b'e' | b'E' => self.parse_exponent(positive, significand, exponent),
            _ => self.f64_from_parts(positive, significand, exponent),
        }
    }