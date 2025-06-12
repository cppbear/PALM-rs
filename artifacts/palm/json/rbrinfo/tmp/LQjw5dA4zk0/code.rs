fn parse_number(&mut self, positive: bool, significand: u64) -> Result<ParserNumber> {
        Ok(match tri!(self.peek_or_null()) {
            b'.' => ParserNumber::F64(tri!(self.parse_decimal(positive, significand, 0))),
            b'e' | b'E' => ParserNumber::F64(tri!(self.parse_exponent(positive, significand, 0))),
            _ => {
                if positive {
                    ParserNumber::U64(significand)
                } else {
                    let neg = (significand as i64).wrapping_neg();

                    // Convert into a float if we underflow, or on `-0`.
                    if neg >= 0 {
                        ParserNumber::F64(-(significand as f64))
                    } else {
                        ParserNumber::I64(neg)
                    }
                }
            }
        })
    }