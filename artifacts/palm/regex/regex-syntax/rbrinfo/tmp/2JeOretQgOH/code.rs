fn cls_char_count(cls: &hir::ClassUnicode) -> usize {
    cls.iter()
        .map(|&r| 1 + (r.end as u32) - (r.start as u32))
        .sum::<u32>() as usize
}