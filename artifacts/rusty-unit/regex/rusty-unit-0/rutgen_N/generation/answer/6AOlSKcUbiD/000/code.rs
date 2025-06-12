// Answer 0

#[derive(Default)]
struct Parser {
    capture_index: std::cell::Cell<u32>,
}

impl Parser {
    fn new() -> Self {
        Self {
            capture_index: std::cell::Cell::new(0),
        }
    }

    fn capture_index(&self) -> std::cell::Cell<u32> {
        self.capture_index
    }

    fn set_capture_index(&self, index: u32) {
        self.capture_index.set(index);
    }
}

struct Span;

struct CaptureIndexManager {
    parser: Parser,
}

impl CaptureIndexManager {
    fn parser(&self) -> &Parser {
        &self.parser
    }

    fn error(&self, _span: Span, _kind: ast::ErrorKind) -> () {
        // Simulated error handling
    }

    fn next_capture_index(&self, span: Span) -> Result<u32, String> {
        let current = self.parser().capture_index.get();
        let i = current.checked_add(1).ok_or_else(|| {
            self.error(span, ast::ErrorKind::CaptureLimitExceeded);
            "Capture limit exceeded".to_string()
        })?;
        self.parser().set_capture_index(i);
        Ok(i)
    }
}

#[derive(Debug)]
enum ast {
    ErrorKind,
}

impl ast {
    const CaptureLimitExceeded: ErrorKind = ErrorKind;
}

#[test]
fn test_next_capture_index_within_limit() {
    let manager = CaptureIndexManager {
        parser: Parser::new(),
    };
    let span = Span;
    let result = manager.next_capture_index(span);
    assert_eq!(result, Ok(1));
    let result = manager.next_capture_index(span);
    assert_eq!(result, Ok(2));
}

#[test]
#[should_panic(expected = "Capture limit exceeded")]
fn test_next_capture_index_exceeds_limit() {
    let manager = CaptureIndexManager {
        parser: Parser {
            capture_index: std::cell::Cell::new(u32::MAX),
        },
    };
    let span = Span;
    // This should panic due to exceeding the limit.
    let _ = manager.next_capture_index(span);
}

