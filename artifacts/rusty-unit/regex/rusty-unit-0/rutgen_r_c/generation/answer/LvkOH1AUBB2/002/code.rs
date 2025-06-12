// Answer 0

#[test]
fn test_notate_line_with_single_span() {
    struct MockSpan {
        start: Position,
        end: Position,
    }
    
    struct MockSpans {
        by_line: Vec<Vec<MockSpan>>,
        line_number_width: usize,
    }

    impl MockSpans {
        fn line_number_padding(&self) -> usize {
            if self.line_number_width == 0 {
                4
            } else {
                2 + self.line_number_width
            }
        }

        fn notate_line(&self, i: usize) -> Option<String> {
            let spans = &self.by_line[i];
            if spans.is_empty() {
                return None;
            }
            let mut notes = String::new();
            for _ in 0..self.line_number_padding() {
                notes.push(' ');
            }
            let mut pos = 0;
            for span in spans {
                for _ in pos..(span.start.column - 1) {
                    notes.push(' ');
                    pos += 1;
                }
                let note_len = span.end.column.saturating_sub(span.start.column);
                for _ in 0..cmp::max(1, note_len) {
                    notes.push('^');
                    pos += 1;
                }
            }
            Some(notes)
        }
    }

    let span = MockSpan { start: Position { column: 5 }, end: Position { column: 9 } };
    let spans = MockSpans { by_line: vec![vec![span]], line_number_width: 2 };
    
    let result = spans.notate_line(0);
    assert_eq!(result, Some("  ^^^^".to_string()));
}

#[test]
fn test_notate_line_with_multiple_spans() {
    struct MockSpan {
        start: Position,
        end: Position,
    }

    struct MockSpans {
        by_line: Vec<Vec<MockSpan>>,
        line_number_width: usize,
    }

    impl MockSpans {
        fn line_number_padding(&self) -> usize {
            if self.line_number_width == 0 {
                4
            } else {
                2 + self.line_number_width
            }
        }

        fn notate_line(&self, i: usize) -> Option<String> {
            let spans = &self.by_line[i];
            if spans.is_empty() {
                return None;
            }
            let mut notes = String::new();
            for _ in 0..self.line_number_padding() {
                notes.push(' ');
            }
            let mut pos = 0;
            for span in spans {
                for _ in pos..(span.start.column - 1) {
                    notes.push(' ');
                    pos += 1;
                }
                let note_len = span.end.column.saturating_sub(span.start.column);
                for _ in 0..cmp::max(1, note_len) {
                    notes.push('^');
                    pos += 1;
                }
            }
            Some(notes)
        }
    }

    let span1 = MockSpan { start: Position { column: 5 }, end: Position { column: 7 } };
    let span2 = MockSpan { start: Position { column: 10 }, end: Position { column: 12 } };
    let spans = MockSpans { by_line: vec![vec![span1, span2]], line_number_width: 2 };

    let result = spans.notate_line(0);
    assert_eq!(result, Some("  ^^^   ^^^".to_string()));
}

#[test]
fn test_notate_line_with_empty_spans() {
    struct MockSpan {
        start: Position,
        end: Position,
    }

    struct MockSpans {
        by_line: Vec<Vec<MockSpan>>,
        line_number_width: usize,
    }

    impl MockSpans {
        fn line_number_padding(&self) -> usize {
            if self.line_number_width == 0 {
                4
            } else {
                2 + self.line_number_width
            }
        }

        fn notate_line(&self, i: usize) -> Option<String> {
            let spans = &self.by_line[i];
            if spans.is_empty() {
                return None;
            }
            let mut notes = String::new();
            for _ in 0..self.line_number_padding() {
                notes.push(' ');
            }
            let mut pos = 0;
            for span in spans {
                for _ in pos..(span.start.column - 1) {
                    notes.push(' ');
                    pos += 1;
                }
                let note_len = span.end.column.saturating_sub(span.start.column);
                for _ in 0..cmp::max(1, note_len) {
                    notes.push('^');
                    pos += 1;
                }
            }
            Some(notes)
        }
    }

    let spans = MockSpans { by_line: vec![vec![]], line_number_width: 2 };
    
    let result = spans.notate_line(0);
    assert_eq!(result, None);
}

#[test]
fn test_notate_line_with_invalid_index() {
    struct MockSpan {
        start: Position,
        end: Position,
    }

    struct MockSpans {
        by_line: Vec<Vec<MockSpan>>,
        line_number_width: usize,
    }

    impl MockSpans {
        fn line_number_padding(&self) -> usize {
            if self.line_number_width == 0 {
                4
            } else {
                2 + self.line_number_width
            }
        }

        fn notate_line(&self, i: usize) -> Option<String> {
            if i >= self.by_line.len() {
                panic!("Index out of bounds");
            }
            let spans = &self.by_line[i];
            if spans.is_empty() {
                return None;
            }
            let mut notes = String::new();
            for _ in 0..self.line_number_padding() {
                notes.push(' ');
            }
            let mut pos = 0;
            for span in spans {
                for _ in pos..(span.start.column - 1) {
                    notes.push(' ');
                    pos += 1;
                }
                let note_len = span.end.column.saturating_sub(span.start.column);
                for _ in 0..cmp::max(1, note_len) {
                    notes.push('^');
                    pos += 1;
                }
            }
            Some(notes)
        }
    }

    let spans = MockSpans { by_line: vec![vec![MockSpan { start: Position { column: 5 }, end: Position { column: 10 } }]], line_number_width: 2 };

    let panic_result = std::panic::catch_unwind(|| {
        spans.notate_line(1);
    });

    assert!(panic_result.is_err());
}

