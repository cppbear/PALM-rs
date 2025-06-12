fn hir_ascii_class_bytes(kind: &ast::ClassAsciiKind) -> hir::ClassBytes {
    let ranges: Vec<_> = ascii_class(kind).iter().cloned().map(|(s, e)| {
        hir::ClassBytesRange::new(s as u8, e as u8)
    }).collect();
    hir::ClassBytes::new(ranges)
}