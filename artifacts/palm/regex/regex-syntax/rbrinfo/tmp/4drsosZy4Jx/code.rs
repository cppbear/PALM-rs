pub fn concat(mut exprs: Vec<Hir>) -> Hir {
        match exprs.len() {
            0 => Hir::empty(),
            1 => exprs.pop().unwrap(),
            _ => {
                let mut info = HirInfo::new();
                info.set_always_utf8(true);
                info.set_all_assertions(true);
                info.set_any_anchored_start(false);
                info.set_any_anchored_end(false);
                info.set_match_empty(true);

                // Some attributes require analyzing all sub-expressions.
                for e in &exprs {
                    let x = info.is_always_utf8() && e.is_always_utf8();
                    info.set_always_utf8(x);

                    let x = info.is_all_assertions() && e.is_all_assertions();
                    info.set_all_assertions(x);

                    let x =
                        info.is_any_anchored_start()
                        || e.is_any_anchored_start();
                    info.set_any_anchored_start(x);

                    let x =
                        info.is_any_anchored_end()
                        || e.is_any_anchored_end();
                    info.set_any_anchored_end(x);

                    let x = info.is_match_empty() && e.is_match_empty();
                    info.set_match_empty(x);
                }
                // Anchored attributes require something slightly more
                // sophisticated. Normally, WLOG, to determine whether an
                // expression is anchored to the start, we'd only need to check
                // the first expression of a concatenation. However,
                // expressions like `$\b^` are still anchored to the start,
                // but the first expression in the concatenation *isn't*
                // anchored to the start. So the "first" expression to look at
                // is actually one that is either not an assertion or is
                // specifically the StartText assertion.
                info.set_anchored_start(
                    exprs.iter()
                        .take_while(|e| {
                            e.is_anchored_start() || e.is_all_assertions()
                        })
                        .any(|e| {
                            e.is_anchored_start()
                        }));
                // Similarly for the end anchor, but in reverse.
                info.set_anchored_end(
                    exprs.iter()
                        .rev()
                        .take_while(|e| {
                            e.is_anchored_end() || e.is_all_assertions()
                        })
                        .any(|e| {
                            e.is_anchored_end()
                        }));
                Hir {
                    kind: HirKind::Concat(exprs),
                    info: info,
                }
            }
        }
    }