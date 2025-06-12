fn parse_long_integer(&mut self, positive: bool, significand: u64) -> Result<f64> {
        let mut exponent = 0;
        loop {
            match tri!(self.peek_or_null()) {
                b'0'..=b'9' => {
                    self.eat_char();
                    // This could overflow... if your integer is gigabytes long.
                    // Ignore that possibility.
                    exponent += 1;
                }
                b'.' => {
                    return self.parse_decimal(positive, significand, exponent);
                }
                b'e' | b'E' => {
                    return self.parse_exponent(positive, significand, exponent);
                }
                _ => {
                    return self.f64_from_parts(positive, significand, exponent);
                }
            }
        }
    }