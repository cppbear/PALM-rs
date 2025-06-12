fn prefixes(expr: &Hir, lits: &mut Literals) {
    match *expr.kind() {
        HirKind::Literal(hir::Literal::Unicode(c)) => {
            let mut buf = [0; 4];
            lits.cross_add(c.encode_utf8(&mut buf).as_bytes());
        }
        HirKind::Literal(hir::Literal::Byte(b)) => {
            lits.cross_add(&[b]);
        }
        HirKind::Class(hir::Class::Unicode(ref cls)) => {
            if !lits.add_char_class(cls) {
                lits.cut();
            }
        }
        HirKind::Class(hir::Class::Bytes(ref cls)) => {
            if !lits.add_byte_class(cls) {
                lits.cut();
            }
        }
        HirKind::Group(hir::Group { ref hir, .. }) => {
            prefixes(&**hir, lits);
        }
        HirKind::Repetition(ref x) => {
            match x.kind {
                hir::RepetitionKind::ZeroOrOne => {
                    repeat_zero_or_one_literals(&x.hir, lits, prefixes);
                }
                hir::RepetitionKind::ZeroOrMore => {
                    repeat_zero_or_more_literals(&x.hir, lits, prefixes);
                }
                hir::RepetitionKind::OneOrMore => {
                    repeat_one_or_more_literals(&x.hir, lits, prefixes);
                }
                hir::RepetitionKind::Range(ref rng) => {
                    let (min, max) = match *rng {
                        hir::RepetitionRange::Exactly(m) => {
                            (m, Some(m))
                        }
                        hir::RepetitionRange::AtLeast(m) => {
                            (m, None)
                        }
                        hir::RepetitionRange::Bounded(m, n) => {
                            (m, Some(n))
                        }
                    };
                    repeat_range_literals(
                        &x.hir, min, max, x.greedy, lits, prefixes)
                }
            }
        }
        HirKind::Concat(ref es) if es.is_empty() => {}
        HirKind::Concat(ref es) if es.len() == 1 => prefixes(&es[0], lits),
        HirKind::Concat(ref es) => {
            for e in es {
                if let HirKind::Anchor(hir::Anchor::StartText) = *e.kind() {
                    if !lits.is_empty() {
                        lits.cut();
                        break;
                    }
                    lits.add(Literal::empty());
                    continue;
                }
                let mut lits2 = lits.to_empty();
                prefixes(e, &mut lits2);
                if !lits.cross_product(&lits2) || !lits2.any_complete() {
                    // If this expression couldn't yield any literal that
                    // could be extended, then we need to quit. Since we're
                    // short-circuiting, we also need to freeze every member.
                    lits.cut();
                    break;
                }
            }
        }
        HirKind::Alternation(ref es) => {
            alternate_literals(es, lits, prefixes);
        }
        _ => lits.cut(),
    }
}