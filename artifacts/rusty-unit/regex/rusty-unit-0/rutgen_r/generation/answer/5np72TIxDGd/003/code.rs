// Answer 0

#[derive(Debug)]
struct Span;
struct Hir;
mod unicode {
    pub fn contains_simple_case_mapping(c: char, _: char) -> bool {
        c.is_ascii_alphabetic() || c.is_ascii_digit()
    }
}
struct Flags {
    unicode: bool,
}
struct Context {
    flags: Flags,
}
impl Context {
    pub fn flags(&self) -> &Flags {
        &self.flags
    }
    pub fn hir_from_char(&self, _: Span, c: char) -> Result<Hir, ()> {
        Ok(Hir)
    }
}
mod hir {
    #[derive(Debug)]
    pub struct Class;
    #[derive(Debug)]
    pub struct ClassBytes {
        ranges: Vec<ClassBytesRange>,
    }
    impl ClassBytes {
        pub fn new(ranges: Vec<ClassBytesRange>) -> Self {
            ClassBytes { ranges }
        }
        pub fn case_fold_simple(&mut self) {}
    }
    #[derive(Debug)]
    pub struct ClassBytesRange;
    impl ClassBytesRange {
        pub fn new(start: u8, end: u8) -> Self {
            ClassBytesRange
        }
    }
    #[derive(Debug)]
    pub struct ClassUnicode;
    #[derive(Debug)]
    pub struct ClassUnicodeRange;
    impl ClassUnicode {
        pub fn new(_: Vec<ClassUnicodeRange>) -> Self {
            ClassUnicode
        }
        pub fn case_fold_simple(&mut self) {}
    }
}
fn hir_from_char_case_insensitive(ctx: &Context, span: Span, c: char) -> Result<Hir, ()> {
    if !unicode::contains_simple_case_mapping(c, c) {
        return ctx.hir_from_char(span, c);
    }
    if ctx.flags().unicode {
        let mut cls = hir::ClassUnicode::new(vec![hir::ClassUnicodeRange::new()]);
        cls.case_fold_simple();
        Ok(Hir::class(hir::Class::Unicode(cls)))
    } else {
        if c.len_utf8() > 1 {
            return Err(());
        }
        let mut cls = hir::ClassBytes::new(vec![hir::ClassBytesRange::new(c as u8, c as u8)]);
        cls.case_fold_simple();
        Ok(Hir::class(hir::Class::Bytes(cls)))
    }
}

#[test]
fn test_hir_from_char_case_insensitive() {
    let ctx = Context { flags: Flags { unicode: false } };
    let span = Span;
    let c = 'a'; // single ASCII character which meets the constraints
    let result = hir_from_char_case_insensitive(&ctx, span, c);
    assert!(result.is_ok());
    // Further assertions can be added to check the output value.
}

