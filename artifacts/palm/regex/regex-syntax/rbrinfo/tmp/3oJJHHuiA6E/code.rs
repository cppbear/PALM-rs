pub fn class(class: Class) -> Hir {
        let mut info = HirInfo::new();
        info.set_always_utf8(class.is_always_utf8());
        info.set_all_assertions(false);
        info.set_anchored_start(false);
        info.set_anchored_end(false);
        info.set_any_anchored_start(false);
        info.set_any_anchored_end(false);
        info.set_match_empty(false);
        Hir {
            kind: HirKind::Class(class),
            info: info,
        }
    }