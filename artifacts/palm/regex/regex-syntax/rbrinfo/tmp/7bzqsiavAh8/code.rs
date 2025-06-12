fn repeat_range_literals<F: FnMut(&Hir, &mut Literals)>(
    e: &Hir,
    min: u32,
    max: Option<u32>,
    greedy: bool,
    lits: &mut Literals,
    mut f: F,
) {
    if min == 0 {
        // This is a bit conservative. If `max` is set, then we could
        // treat this as a finite set of alternations. For now, we
        // just treat it as `e*`.
        f(&Hir::repetition(hir::Repetition {
            kind: hir::RepetitionKind::ZeroOrMore,
            greedy: greedy,
            hir: Box::new(e.clone()),
        }), lits);
    } else {
        if min > 0 {
            let n = cmp::min(lits.limit_size, min as usize);
            let es = iter::repeat(e.clone()).take(n).collect();
            f(&Hir::concat(es), lits);
            if n < min as usize || lits.contains_empty() {
                lits.cut();
            }
        }
        if max.map_or(true, |max| min < max) {
            lits.cut();
        }
    }
}