// Answer 0

#[test]
fn test_prefixes_empty_unicode_class() {
    let cls = Class::Unicode(ClassUnicode { set: IntervalSet::empty() });
    let hir = Hir::class(cls);
    let mut lits = Literals::empty().set_limit_size(0);
    prefixes(&hir, &mut lits);
}

#[test]
fn test_prefixes_empty_bytes_class() {
    let cls = Class::Bytes(ClassBytes { set: IntervalSet::empty() });
    let hir = Hir::class(cls);
    let mut lits = Literals::empty().set_limit_size(0);
    prefixes(&hir, &mut lits);
}

#[test]
fn test_prefixes_non_empty_unicode_class_with_limit_zero() {
    let cls = Class::Unicode(ClassUnicode { set: IntervalSet::new(vec![UnicodeRange::new('a' as u32, 'a' as u32)]) });
    let hir = Hir::class(cls);
    let mut lits = Literals::empty().set_limit_size(0);
    prefixes(&hir, &mut lits);
}

#[test]
fn test_prefixes_non_empty_bytes_class_with_limit_zero() {
    let cls = Class::Bytes(ClassBytes { set: IntervalSet::new(vec![BytesRange::new(97, 97)]) });
    let hir = Hir::class(cls);
    let mut lits = Literals::empty().set_limit_size(0);
    prefixes(&hir, &mut lits);
}

#[test]
fn test_prefixes_unicode_class_with_empty_lits() {
    let cls = Class::Unicode(ClassUnicode { set: IntervalSet::new(vec![UnicodeRange::new('a' as u32, 'a' as u32)]) });
    let hir = Hir::class(cls);
    let mut lits = Literals { lits: vec![], limit_size: 0, limit_class: 1 };
    prefixes(&hir, &mut lits);
}

#[test]
fn test_prefixes_bytes_class_with_empty_lits() {
    let cls = Class::Bytes(ClassBytes { set: IntervalSet::new(vec![BytesRange::new(97, 97)]) });
    let hir = Hir::class(cls);
    let mut lits = Literals { lits: vec![], limit_size: 0, limit_class: 1 };
    prefixes(&hir, &mut lits);
}

