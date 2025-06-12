fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        struct LookForDecimalPoint<'f, 'a> {
            formatter: &'f mut fmt::Formatter<'a>,
            has_decimal_point: bool,
        }

        impl<'f, 'a> fmt::Write for LookForDecimalPoint<'f, 'a> {
            fn write_str(&mut self, fragment: &str) -> fmt::Result {
                self.has_decimal_point |= fragment.contains('.');
                self.formatter.write_str(fragment)
            }

            fn write_char(&mut self, ch: char) -> fmt::Result {
                self.has_decimal_point |= ch == '.';
                self.formatter.write_char(ch)
            }
        }

        if self.0.is_finite() {
            let mut writer = LookForDecimalPoint {
                formatter,
                has_decimal_point: false,
            };
            tri!(write!(writer, "{}", self.0));
            if !writer.has_decimal_point {
                tri!(formatter.write_str(".0"));
            }
        } else {
            tri!(write!(formatter, "{}", self.0));
        }
        Ok(())
    }