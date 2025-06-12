pub fn group(group: Group) -> Hir {
        let mut info = HirInfo::new();
        info.set_always_utf8(group.hir.is_always_utf8());
        info.set_all_assertions(group.hir.is_all_assertions());
        info.set_anchored_start(group.hir.is_anchored_start());
        info.set_anchored_end(group.hir.is_anchored_end());
        info.set_any_anchored_start(group.hir.is_any_anchored_start());
        info.set_any_anchored_end(group.hir.is_any_anchored_end());
        info.set_match_empty(group.hir.is_match_empty());
        Hir {
            kind: HirKind::Group(group),
            info: info,
        }
    }