// Answer 0

#[derive(Debug)]
struct Flags {
    unicode: bool,
    dot_matches_new_line: bool,
}

impl Flags {
    fn new(unicode: bool, dot_matches_new_line: bool) -> Self {
        Self {
            unicode,
            dot_matches_new_line,
        }
    }

    fn unicode(&self) -> bool {
        self.unicode
    }

    fn dot_matches_new_line(&self) -> bool {
        self.dot_matches_new_line
    }
}

#[derive(Debug)]
struct Trans {
    allow_invalid_utf8: bool,
}

impl Trans {
    fn new(allow_invalid_utf8: bool) -> Self {
        Self { allow_invalid_utf8 }
    }
}

#[derive(Debug)]
struct Span;

#[derive(Debug)]
enum ErrorKind {
    InvalidUtf8,
}

#[derive(Debug)]
struct Hir {
    value: String,
}

impl Hir {
    fn any(_: bool) -> Self {
        Hir {
            value: "any".to_string(),
        }
    }

    fn dot(_: bool) -> Self {
        Hir {
            value: "dot".to_string(),
        }
    }
}

#[derive(Debug)]
struct Context {
    flags: Flags,
    trans: Trans,
}

impl Context {
    fn flags(&self) -> &Flags {
        &self.flags
    }

    fn trans(&self) -> &Trans {
        &self.trans
    }

    fn error(&self, _: Span, kind: ErrorKind) -> String {
        format!("{:?}", kind)
    }

    fn hir_dot(&self, span: Span) -> Result<Hir, String> {
        let unicode = self.flags().unicode();
        if !unicode && !self.trans().allow_invalid_utf8 {
            return Err(self.error(span, ErrorKind::InvalidUtf8));
        }
        Ok(if self.flags().dot_matches_new_line() {
            Hir::any(!unicode)
        } else {
            Hir::dot(!unicode)
        })
    }
}

#[test]
fn test_hir_dot_with_unicode_and_dot_matches_new_line() {
    let flags = Flags::new(true, true);
    let trans = Trans::new(true);
    let context = Context { flags, trans };
    let span = Span;

    let result = context.hir_dot(span).unwrap();

    assert_eq!(result.value, "any");
}

