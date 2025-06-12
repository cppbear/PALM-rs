fn line_number_padding(&self) -> usize {
        if self.line_number_width == 0 {
            4
        } else {
            2 + self.line_number_width
        }
    }