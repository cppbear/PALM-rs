pub fn alternation(mut exprs: Vec<Hir>) -> Hir {
        match exprs.len() {
            0 => Hir::empty(),
            1 => exprs.pop().unwrap(),
            _ => {
                let mut info = HirInfo::new();
                info.set_always_utf8(true);
                info.set_all_assertions(true);
                info.set_anchored_start(true);
                info.set_anchored_end(true);
                info.set_any_anchored_start(false);
                info.set_any_anchored_end(false);
                info.set_match_empty(false);

                // Some attributes require analyzing all sub-expressions.
                for e in &exprs {
                    let x = info.is_always_utf8() && e.is_always_utf8();
                    info.set_always_utf8(x);

                    let x = info.is_all_assertions() && e.is_all_assertions();
                    info.set_all_assertions(x);

                    let x = info.is_anchored_start() && e.is_anchored_start();
                    info.set_anchored_start(x);

                    let x = info.is_anchored_end() && e.is_anchored_end();
                    info.set_anchored_end(x);

                    let x =
                        info.is_any_anchored_start()
                        || e.is_any_anchored_start();
                    info.set_any_anchored_start(x);

                    let x =
                        info.is_any_anchored_end()
                        || e.is_any_anchored_end();
                    info.set_any_anchored_end(x);

                    let x = info.is_match_empty() || e.is_match_empty();
                    info.set_match_empty(x);
                }
                Hir {
                    kind: HirKind::Alternation(exprs),
                    info: info,
                }
            }
        }
    }