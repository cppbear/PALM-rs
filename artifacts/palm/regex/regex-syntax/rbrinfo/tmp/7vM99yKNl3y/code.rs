fn from_formatter<'e, E: fmt::Display>(
        fmter: &'p Formatter<'e, E>,
    ) -> Spans<'p> {
        let mut line_count = fmter.pattern.lines().count();
        // If the pattern ends with a `\n` literal, then our line count is
        // off by one, since a span can occur immediately after the last `\n`,
        // which is consider to be an additional line.
        if fmter.pattern.ends_with('\n') {
            line_count += 1;
        }
        let line_number_width =
            if line_count <= 1 {
                0
            } else {
                line_count.to_string().len()
            };
        let mut spans = Spans {
            pattern: &fmter.pattern,
            line_number_width: line_number_width,
            by_line: vec![vec![]; line_count],
            multi_line: vec![],
        };
        spans.add(fmter.span.clone());
        if let Some(span) = fmter.aux_span {
            spans.add(span.clone());
        }
        spans
    }