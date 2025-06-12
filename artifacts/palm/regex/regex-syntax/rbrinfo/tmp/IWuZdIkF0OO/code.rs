fn suffixes(expr: &Hir, lits: &mut Literals) {
    match *expr.kind() {
        HirKind::Literal(hir::Literal::Unicode(c)) => {
            let mut buf = [0u8; 4];
            let i = c.encode_utf8(&mut buf).len();
            let mut buf = &mut buf[..i];
            buf.reverse();
            lits.cross_add(buf);
        }
        HirKind::Literal(hir::Literal::Byte(b)) => {
            lits.cross_add(&[b]);
        }
        HirKind::Class(hir::Class::Unicode(ref cls)) => {
            if !lits.add_char_class_reverse(cls) {
                lits.cut();
            }
        }
        HirKind::Class(hir::Class::Bytes(ref cls)) => {
            if !lits.add_byte_class(cls) {
                lits.cut();
            }
        }
        HirKind::Group(hir::Group { ref hir, .. }) => {
            suffixes(&**hir, lits);
        }
        HirKind::Repetition(ref x) => {
            match x.kind {
                hir::RepetitionKind::ZeroOrOne => {
                    repeat_zero_or_one_literals(&x.hir, lits, suffixes);
                }
                hir::RepetitionKind::ZeroOrMore => {
                    repeat_zero_or_more_literals(&x.hir, lits, suffixes);
                }
                hir::RepetitionKind::OneOrMore => {
                    repeat_one_or_more_literals(&x.hir, lits, suffixes);
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
                        &x.hir, min, max, x.greedy, lits, suffixes)
                }
            }
        }
        HirKind::Concat(ref es) if es.is_empty() => {}
        HirKind::Concat(ref es) if es.len() == 1 => suffixes(&es[0], lits),
        HirKind::Concat(ref es) => {
            for e in es.iter().rev() {
                if let HirKind::Anchor(hir::Anchor::EndText) = *e.kind() {
                    if !lits.is_empty() {
                        lits.cut();
                        break;
                    }
                    lits.add(Literal::empty());
                    continue;
                }
                let mut lits2 = lits.to_empty();
                suffixes(e, &mut lits2);
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
            alternate_literals(es, lits, suffixes);
        }
        _ => lits.cut(),
    }
}