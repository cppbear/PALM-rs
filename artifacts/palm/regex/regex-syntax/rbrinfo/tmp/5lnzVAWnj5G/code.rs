fn drop(&mut self) {
        use std::mem;

        match *self.kind() {
            HirKind::Empty
            | HirKind::Literal(_)
            | HirKind::Class(_)
            | HirKind::Anchor(_)
            | HirKind::WordBoundary(_) => return,
            HirKind::Group(ref x) if !x.hir.kind.has_subexprs() => return,
            HirKind::Repetition(ref x) if !x.hir.kind.has_subexprs() => return,
            HirKind::Concat(ref x) if x.is_empty() => return,
            HirKind::Alternation(ref x) if x.is_empty() => return,
            _ => {}
        }

        let mut stack = vec![mem::replace(self, Hir::empty())];
        while let Some(mut expr) = stack.pop() {
            match expr.kind {
                HirKind::Empty
                | HirKind::Literal(_)
                | HirKind::Class(_)
                | HirKind::Anchor(_)
                | HirKind::WordBoundary(_) => {}
                HirKind::Group(ref mut x) => {
                    stack.push(mem::replace(&mut x.hir, Hir::empty()));
                }
                HirKind::Repetition(ref mut x) => {
                    stack.push(mem::replace(&mut x.hir, Hir::empty()));
                }
                HirKind::Concat(ref mut x) => {
                    stack.extend(x.drain(..));
                }
                HirKind::Alternation(ref mut x) => {
                    stack.extend(x.drain(..));
                }
            }
        }
    }