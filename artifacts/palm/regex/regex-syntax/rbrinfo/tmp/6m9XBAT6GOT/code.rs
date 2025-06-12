pub fn repetition(rep: Repetition) -> Hir {
        let mut info = HirInfo::new();
        info.set_always_utf8(rep.hir.is_always_utf8());
        info.set_all_assertions(rep.hir.is_all_assertions());
        // If this operator can match the empty string, then it can never
        // be anchored.
        info.set_anchored_start(
            !rep.is_match_empty() && rep.hir.is_anchored_start()
        );
        info.set_anchored_end(
            !rep.is_match_empty() && rep.hir.is_anchored_end()
        );
        info.set_any_anchored_start(rep.hir.is_any_anchored_start());
        info.set_any_anchored_end(rep.hir.is_any_anchored_end());
        info.set_match_empty(rep.is_match_empty() || rep.hir.is_match_empty());
        Hir {
            kind: HirKind::Repetition(rep),
            info: info,
        }
    }