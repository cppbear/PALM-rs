pub fn literal(lit: Literal) -> Hir {
        if let Literal::Byte(b) = lit {
            assert!(b > 0x7F);
        }

        let mut info = HirInfo::new();
        info.set_always_utf8(lit.is_unicode());
        info.set_all_assertions(false);
        info.set_anchored_start(false);
        info.set_anchored_end(false);
        info.set_any_anchored_start(false);
        info.set_any_anchored_end(false);
        info.set_match_empty(false);
        Hir {
            kind: HirKind::Literal(lit),
            info: info,
        }
    }