// Answer 0

#[derive(Default)]
struct Flags {
    case_insensitive: bool,
}

impl Flags {
    fn case_insensitive(&self) -> bool {
        self.case_insensitive
    }
}

struct ClassUnicode {
    is_negated: bool,
    is_case_folded: bool,
}

impl ClassUnicode {
    fn new() -> Self {
        Self {
            is_negated: false,
            is_case_folded: false,
        }
    }

    fn negate(&mut self) {
        self.is_negated = true;
    }

    fn case_fold_simple(&mut self) {
        self.is_case_folded = true;
    }
}

struct Hir {
    flags: Flags,
}

impl Hir {
    fn new(flags: Flags) -> Self {
        Self { flags }
    }

    fn unicode_fold_and_negate(
        &self,
        negated: bool,
        class: &mut ClassUnicode,
    ) {
        if self.flags.case_insensitive() {
            class.case_fold_simple();
        }
        if negated {
            class.negate();
        }
    }
}

#[test]
fn test_unicode_fold_and_negate_case_insensitive_negated() {
    let flags = Flags { case_insensitive: true };
    let hir = Hir::new(flags);
    let mut class = ClassUnicode::new();
    
    hir.unicode_fold_and_negate(true, &mut class);
    
    assert!(class.is_case_folded);
    assert!(class.is_negated);
}

#[test]
fn test_unicode_fold_and_negate_case_insensitive_non_negated() {
    let flags = Flags { case_insensitive: true };
    let hir = Hir::new(flags);
    let mut class = ClassUnicode::new();

    hir.unicode_fold_and_negate(false, &mut class);
    
    assert!(class.is_case_folded);
    assert!(!class.is_negated);
}

#[test]
fn test_unicode_fold_and_negate_case_sensitive_negated() {
    let flags = Flags { case_insensitive: false };
    let hir = Hir::new(flags);
    let mut class = ClassUnicode::new();
    
    hir.unicode_fold_and_negate(true, &mut class);
    
    assert!(!class.is_case_folded);
    assert!(class.is_negated);
}

#[test]
fn test_unicode_fold_and_negate_case_sensitive_non_negated() {
    let flags = Flags { case_insensitive: false };
    let hir = Hir::new(flags);
    let mut class = ClassUnicode::new();

    hir.unicode_fold_and_negate(false, &mut class);
    
    assert!(!class.is_case_folded);
    assert!(!class.is_negated);
}

