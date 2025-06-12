// Answer 0

#[derive(Default)]
struct Span {
    start: Position,
    end: Position,
}

#[derive(Default)]
struct Position {
    column: usize,
}

struct Notation {
    by_line: Vec<Vec<Span>>,
}

impl Notation {
    fn line_number_padding(&self) -> usize {
        0 // Placeholder for simplicity; adjust as per actual implementation
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
fn test_notate_line_with_no_spans() {
    let notation = Notation {
        by_line: vec![vec![]],
    };
    assert_eq!(notation.notate_line(0), None);
}

#[test]
fn test_notate_line_with_single_span() {
    let notation = Notation {
        by_line: vec![vec![Span { start: Position { column: 3 }, end: Position { column: 6 } }]],
    };
    assert_eq!(notation.notate_line(0), Some("  ^^^".to_string()));
}

#[test]
fn test_notate_line_with_multiple_spans() {
    let notation = Notation {
        by_line: vec![
            vec![
                Span { start: Position { column: 2 }, end: Position { column: 4 } },
                Span { start: Position { column: 6 }, end: Position { column: 8 } },
            ],
        ],
    };
    assert_eq!(notation.notate_line(0), Some("  ^^  ^^".to_string()));
}

#[test]
fn test_notate_line_with_span_at_start() {
    let notation = Notation {
        by_line: vec![vec![Span { start: Position { column: 1 }, end: Position { column: 3 } }]],
    };
    assert_eq!(notation.notate_line(0), Some("^ ^".to_string()));
}

#[test]
fn test_notate_line_with_span_at_end() {
    let notation = Notation {
        by_line: vec![vec![Span { start: Position { column: 4 }, end: Position { column: 4 } }]],
    };
    assert_eq!(notation.notate_line(0), Some("   ^".to_string()));
}

