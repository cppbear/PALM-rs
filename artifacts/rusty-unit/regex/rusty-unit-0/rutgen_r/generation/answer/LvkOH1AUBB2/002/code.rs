// Answer 0

#[derive(Default)]
struct Span {
    start: Column,
    end: Column,
}

#[derive(Default)]
struct Column {
    column: usize,
}

struct Notation {
    by_line: Vec<Vec<Span>>,
}

impl Notation {
    fn line_number_padding(&self) -> usize {
        4 // example padding
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
            for _ in 0..std::cmp::max(1, note_len) {
                notes.push('^');
                pos += 1;
            }
        }
        Some(notes)
    }
}

#[test]
fn test_notate_line_with_non_empty_spans() {
    let notation = Notation {
        by_line: vec![
            vec![],
            vec![Span { start: Column { column: 5 }, end: Column { column: 10 } }],
        ],
    };

    let result = notation.notate_line(1);
    assert_eq!(result, Some("    ^^^^^".to_string()));
}

#[test]
fn test_notate_line_with_no_spans() {
    let notation = Notation {
        by_line: vec![
            vec![Span { start: Column { column: 2 }, end: Column { column: 4 } }],
            vec![],
        ],
    };

    let result = notation.notate_line(1);
    assert_eq!(result, None);
}

#[test]
fn test_notate_line_with_multiple_span_columns() {
    let notation = Notation {
        by_line: vec![
            vec![Span { start: Column { column: 3 }, end: Column { column: 5 } },
                 Span { start: Column { column: 8 }, end: Column { column: 12 } }],
        ],
    };

    let result = notation.notate_line(0);
    assert_eq!(result, Some("   ^^    ^^^^".to_string()));
}

#[test]
fn test_notate_line_with_padding() {
    let notation = Notation {
        by_line: vec![
            vec![Span { start: Column { column: 1 }, end: Column { column: 1 } }],
        ],
    };

    let result = notation.notate_line(0);
    assert_eq!(result, Some("    ^".to_string()));
}

