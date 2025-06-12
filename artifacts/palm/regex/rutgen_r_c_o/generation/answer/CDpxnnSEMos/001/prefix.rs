// Answer 0

#[test]
fn test_fmt_unrecognized_unescape() {
    let pattern = ".*"; 
    let span = Span {
        start: Position(0),
        end: Position(2),
    };
    let error = Error {
        kind: ErrorKind::UnicodeNotAllowed,
        pattern: pattern.to_string(),
        span,
    };
    let formatter = Formatter {
        pattern,
        err: &error,
        span: &span,
        aux_span: None,
    };
    let _ = formatter.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_invalid_utf8() {
    let pattern = ".*"; 
    let span = Span {
        start: Position(0),
        end: Position(2),
    };
    let error = Error {
        kind: ErrorKind::InvalidUtf8,
        pattern: pattern.to_string(),
        span,
    };
    let formatter = Formatter {
        pattern,
        err: &error,
        span: &span,
        aux_span: None,
    };
    let _ = formatter.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_capture_limit_exceeded() {
    let pattern = "(a|b|c|d|e)";
    let span = Span {
        start: Position(0),
        end: Position(12),
    };
    let error = Error {
        kind: ErrorKind::CaptureLimitExceeded,
        pattern: pattern.to_string(),
        span,
    };
    let formatter = Formatter {
        pattern,
        err: &error,
        span: &span,
        aux_span: None,
    };
    let _ = formatter.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_class_escape_invalid() {
    let pattern = "[a-zA-Z\\]"; 
    let span = Span {
        start: Position(0),
        end: Position(12),
    };
    let error = Error {
        kind: ErrorKind::ClassEscapeInvalid,
        pattern: pattern.to_string(),
        span,
    };
    let formatter = Formatter {
        pattern,
        err: &error,
        span: &span,
        aux_span: None,
    };
    let _ = formatter.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_class_range_invalid() {
    let pattern = "[z-a]"; 
    let span = Span {
        start: Position(0),
        end: Position(6),
    };
    let error = Error {
        kind: ErrorKind::ClassRangeInvalid,
        pattern: pattern.to_string(),
        span,
    };
    let formatter = Formatter {
        pattern,
        err: &error,
        span: &span,
        aux_span: None,
    };
    let _ = formatter.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_class_unclosed() {
    let pattern = "[a-z"; 
    let span = Span {
        start: Position(0),
        end: Position(5),
    };
    let error = Error {
        kind: ErrorKind::ClassUnclosed,
        pattern: pattern.to_string(),
        span,
    };
    let formatter = Formatter {
        pattern,
        err: &error,
        span: &span,
        aux_span: None,
    };
    let _ = formatter.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_decimal_invalid() {
    let pattern = "123"; 
    let span = Span {
        start: Position(0),
        end: Position(3),
    };
    let error = Error {
        kind: ErrorKind::DecimalInvalid,
        pattern: pattern.to_string(),
        span,
    };
    let formatter = Formatter {
        pattern,
        err: &error,
        span: &span,
        aux_span: None,
    };
    let _ = formatter.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_escape_unexpected_eof() {
    let pattern = "\\";
    let span = Span {
        start: Position(0),
        end: Position(1),
    };
    let error = Error {
        kind: ErrorKind::EscapeUnexpectedEof,
        pattern: pattern.to_string(),
        span,
    };
    let formatter = Formatter {
        pattern,
        err: &error,
        span: &span,
        aux_span: None,
    };
    let _ = formatter.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_nest_limit_exceeded() {
    let pattern = "(a(b(c(d(e(f(g(h(i(j(k(l(m(n(o(p(q(r(s(t(u(v(w(x(y(z(aa)))))))))";
    let span = Span {
        start: Position(0),
        end: Position(102),
    };
    let error = Error {
        kind: ErrorKind::NestLimitExceeded(10),
        pattern: pattern.to_string(),
        span,
    };
    let formatter = Formatter {
        pattern,
        err: &error,
        span: &span,
        aux_span: None,
    };
    let _ = formatter.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_repetition_count_invalid() {
    let pattern = "a{5,3}";
    let span = Span {
        start: Position(0),
        end: Position(7),
    };
    let error = Error {
        kind: ErrorKind::RepetitionCountInvalid,
        pattern: pattern.to_string(),
        span,
    };
    let formatter = Formatter {
        pattern,
        err: &error,
        span: &span,
        aux_span: None,
    };
    let _ = formatter.fmt(&mut fmt::Formatter::new());
}

