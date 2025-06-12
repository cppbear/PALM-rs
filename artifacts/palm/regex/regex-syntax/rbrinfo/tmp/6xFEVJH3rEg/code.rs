fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let spans = Spans::from_formatter(self);
        if self.pattern.contains('\n') {
            let divider = repeat_char('~', 79);

            writeln!(f, "regex parse error:")?;
            writeln!(f, "{}", divider)?;
            let notated = spans.notate();
            write!(f, "{}", notated)?;
            writeln!(f, "{}", divider)?;
            // If we have error spans that cover multiple lines, then we just
            // note the line numbers.
            if !spans.multi_line.is_empty() {
                let mut notes = vec![];
                for span in &spans.multi_line {
                    notes.push(format!(
                        "on line {} (column {}) through line {} (column {})",
                        span.start.line, span.start.column,
                        span.end.line, span.end.column - 1));
                }
                writeln!(f, "{}", notes.join("\n"))?;
            }
            write!(f, "error: {}", self.err)?;
        } else {
            writeln!(f, "regex parse error:")?;
            let notated = Spans::from_formatter(self).notate();
            write!(f, "{}", notated)?;
            write!(f, "error: {}", self.err)?;
        }
        Ok(())
    }