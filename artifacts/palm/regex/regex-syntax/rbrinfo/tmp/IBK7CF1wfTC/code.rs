pub fn word_boundary(word_boundary: WordBoundary) -> Hir {
        let mut info = HirInfo::new();
        info.set_always_utf8(true);
        info.set_all_assertions(true);
        info.set_anchored_start(false);
        info.set_anchored_end(false);
        info.set_any_anchored_start(false);
        info.set_any_anchored_end(false);
        // A negated word boundary matches the empty string, but a normal
        // word boundary does not!
        info.set_match_empty(word_boundary.is_negated());
        // Negated ASCII word boundaries can match invalid UTF-8.
        if let WordBoundary::AsciiNegate = word_boundary {
            info.set_always_utf8(false);
        }
        Hir {
            kind: HirKind::WordBoundary(word_boundary),
            info: info,
        }
    }