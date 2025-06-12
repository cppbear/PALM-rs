pub fn hir_class(ranges: &[(char, char)]) -> hir::ClassUnicode {
    let hir_ranges: Vec<hir::ClassUnicodeRange> = ranges
        .iter()
        .map(|&(s, e)| hir::ClassUnicodeRange::new(s, e))
        .collect();
    hir::ClassUnicode::new(hir_ranges)
}