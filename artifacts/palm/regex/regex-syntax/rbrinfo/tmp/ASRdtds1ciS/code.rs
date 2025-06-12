pub fn anchor(anchor: Anchor) -> Hir {
        let mut info = HirInfo::new();
        info.set_always_utf8(true);
        info.set_all_assertions(true);
        info.set_anchored_start(false);
        info.set_anchored_end(false);
        info.set_any_anchored_start(false);
        info.set_any_anchored_end(false);
        info.set_match_empty(true);
        if let Anchor::StartText = anchor {
            info.set_anchored_start(true);
            info.set_any_anchored_start(true);
        }
        if let Anchor::EndText = anchor {
            info.set_anchored_end(true);
            info.set_any_anchored_end(true);
        }
        Hir {
            kind: HirKind::Anchor(anchor),
            info: info,
        }
    }