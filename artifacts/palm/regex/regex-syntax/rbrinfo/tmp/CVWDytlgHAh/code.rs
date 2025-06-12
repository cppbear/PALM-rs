fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
        let c = b as char;
        if c <= 0x7F as char && !c.is_control() && !c.is_whitespace() {
            self.wtr.write_char(c)
        } else {
            write!(self.wtr, "(?-u:\\x{:02X})", b)
        }
    }