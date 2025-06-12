fn notate(&self) -> String {
        let mut notated = String::new();
        for (i, line) in self.pattern.lines().enumerate() {
            if self.line_number_width > 0 {
                notated.push_str(&self.left_pad_line_number(i + 1));
                notated.push_str(": ");
            } else {
                notated.push_str("    ");
            }
            notated.push_str(line);
            notated.push('\n');
            if let Some(notes) = self.notate_line(i) {
                notated.push_str(&notes);
                notated.push('\n');
            }
        }
        notated
    }