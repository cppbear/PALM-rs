// Answer 0

#[test]
fn test_hir_class_basic_ascii() {
    let ranges = &[('\u{0000}', '\u{007F}')];
    hir_class(ranges);
}

#[test]
fn test_hir_class_extended_latin() {
    let ranges = &[('\u{0080}', '\u{00FF}')];
    hir_class(ranges);
}

#[test]
fn test_hir_class_multiple_ranges() {
    let ranges = &[
        ('\u{0000}', '\u{007F}'),
        ('\u{0080}', '\u{00FF}'),
        ('\u{0100}', '\u{017F}')
    ];
    hir_class(ranges);
}

#[test]
fn test_hir_class_full_unicode() {
    let ranges = &[
        ('\u{0000}', '\u{FFFF}'),
    ];
    hir_class(ranges);
}

#[test]
fn test_hir_class_subset() {
    let ranges = &[
        ('\u{0A00}', '\u{0A7F}'),
        ('\u{0C00}', '\u{0C7F}'),
        ('\u{0800}', '\u{083F}')
    ];
    hir_class(ranges);
}

#[test]
fn test_hir_class_edge_range() {
    let ranges = &[('\u{FFFF}', '\u{FFFF}')];
    hir_class(ranges);
}

#[test]
fn test_hir_class_non_continuous_ranges() {
    let ranges = &[
        ('\u{0041}', '\u{0041}'),
        ('\u{007A}', '\u{007A}')
    ];
    hir_class(ranges);
}

#[test]
fn test_hir_class_full_range() {
    let ranges = &[
        ('\u{0000}', '\u{D7FF}'),
        ('\u{E000}', '\u{FFFF}')
    ];
    hir_class(ranges);
}

