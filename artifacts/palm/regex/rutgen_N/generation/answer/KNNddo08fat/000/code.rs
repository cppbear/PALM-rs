// Answer 0

#[derive(Debug)]
struct Span;
#[derive(Debug)]
struct Hir;
#[derive(Debug)]
struct ErrorKind;

impl Hir {
    fn literal(lit: hir::Literal) -> Self {
        Hir
    }
}

mod hir {
    #[derive(Debug)]
    pub struct Literal {
        pub c: char,
    }
    
    impl Literal {
        pub fn Unicode(c: char) -> Self {
            Literal { c }
        }
    }
}

#[derive(Debug)]
struct Flags {
    unicode: bool,
}

impl Flags {
    fn unicode(&self) -> bool {
        self.unicode
    }
}

#[derive(Debug)]
struct MyStruct {
    flags: Flags,
}

impl MyStruct {
    fn flags(&self) -> &Flags {
        &self.flags
    }

    fn error(&self, _span: Span, _kind: ErrorKind) -> String {
        "Error".to_string()
    }

    fn hir_from_char(&self, span: Span, c: char) -> Result<Hir, String> {
        if !self.flags().unicode() && c.len_utf8() > 1 {
            return Err(self.error(span, ErrorKind));
        }
        Ok(Hir::literal(hir::Literal::Unicode(c)))
    }
}

#[test]
fn test_hir_from_char_unicode_allowed() {
    let my_struct = MyStruct {
        flags: Flags { unicode: true },
    };
    let span = Span;
    let result = my_struct.hir_from_char(span, 'a');
    assert!(result.is_ok());
}

#[test]
fn test_hir_from_char_unicode_not_allowed() {
    let my_struct = MyStruct {
        flags: Flags { unicode: false },
    };
    let span = Span;
    let result = my_struct.hir_from_char(span, '♪'); // multi-byte char
    assert!(result.is_err());
}

#[test]
fn test_hir_from_char_single_byte() {
    let my_struct = MyStruct {
        flags: Flags { unicode: false },
    };
    let span = Span;
    let result = my_struct.hir_from_char(span, 'a'); // single-byte char
    assert!(result.is_ok());
}

