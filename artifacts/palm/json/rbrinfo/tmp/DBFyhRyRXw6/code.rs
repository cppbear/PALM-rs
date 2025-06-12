fn parse_exponent(
        &mut self,
        positive: bool,
        significand: u64,
        starting_exp: i32,
    ) -> Result<f64> {
        self.eat_char();

        let positive_exp = match tri!(self.peek_or_null()) {
            b'+' => {
                self.eat_char();
                true
            }
            b'-' => {
                self.eat_char();
                false
            }
            _ => true,
        };

        let next = match tri!(self.next_char()) {
            Some(b) => b,
            None => {
                return Err(self.error(ErrorCode::EofWhileParsingValue));
            }
        };

        // Make sure a digit follows the exponent place.
        let mut exp = match next {
            c @ b'0'..=b'9' => (c - b'0') as i32,
            _ => {
                return Err(self.error(ErrorCode::InvalidNumber));
            }
        };

        while let c @ b'0'..=b'9' = tri!(self.peek_or_null()) {
            self.eat_char();
            let digit = (c - b'0') as i32;

            if overflow!(exp * 10 + digit, i32::MAX) {
                let zero_significand = significand == 0;
                return self.parse_exponent_overflow(positive, zero_significand, positive_exp);
            }

            exp = exp * 10 + digit;
        }

        let final_exp = if positive_exp {
            starting_exp.saturating_add(exp)
        } else {
            starting_exp.saturating_sub(exp)
        };

        self.f64_from_parts(positive, significand, final_exp)
    }