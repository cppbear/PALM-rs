fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let start =
            if !self.start.is_whitespace() && !self.start.is_control() {
                self.start.to_string()
            } else {
                format!("0x{:X}", self.start as u32)
            };
        let end =
            if !self.end.is_whitespace() && !self.end.is_control() {
                self.end.to_string()
            } else {
                format!("0x{:X}", self.end as u32)
            };
        f.debug_struct("ClassUnicodeRange")
         .field("start", &start)
         .field("end", &end)
         .finish()
    }