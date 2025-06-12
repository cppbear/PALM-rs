fn write_literal_char(&mut self, c: char) -> fmt::Result {
        if is_meta_character(c) {
            self.wtr.write_str("\\")?;
        }
        self.wtr.write_char(c)
    }