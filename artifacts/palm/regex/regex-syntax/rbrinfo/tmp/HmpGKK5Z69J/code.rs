fn drop(&mut self) {
        use std::mem;

        match *self {
            ClassSet::Item(ref item) => {
                match *item {
                    ClassSetItem::Empty(_)
                    | ClassSetItem::Literal(_)
                    | ClassSetItem::Range(_)
                    | ClassSetItem::Ascii(_)
                    | ClassSetItem::Unicode(_)
                    | ClassSetItem::Perl(_) => return,
                    ClassSetItem::Bracketed(ref x) => {
                        if x.kind.is_empty() {
                            return;
                        }
                    }
                    ClassSetItem::Union(ref x) => {
                        if x.items.is_empty() {
                            return;
                        }
                    }
                }
            }
            ClassSet::BinaryOp(ref op) => {
                if op.lhs.is_empty() && op.rhs.is_empty() {
                    return;
                }
            }
        }

        let empty_span = || Span::splat(Position::new(0, 0, 0));
        let empty_set = || ClassSet::Item(ClassSetItem::Empty(empty_span()));
        let mut stack = vec![mem::replace(self, empty_set())];
        while let Some(mut set) = stack.pop() {
            match set {
                ClassSet::Item(ref mut item) => {
                    match *item {
                        ClassSetItem::Empty(_)
                        | ClassSetItem::Literal(_)
                        | ClassSetItem::Range(_)
                        | ClassSetItem::Ascii(_)
                        | ClassSetItem::Unicode(_)
                        | ClassSetItem::Perl(_) => {}
                        ClassSetItem::Bracketed(ref mut x) => {
                            stack.push(mem::replace(&mut x.kind, empty_set()));
                        }
                        ClassSetItem::Union(ref mut x) => {
                            stack.extend(
                                x.items.drain(..).map(ClassSet::Item));
                        }
                    }
                }
                ClassSet::BinaryOp(ref mut op) => {
                    stack.push(mem::replace(&mut op.lhs, empty_set()));
                    stack.push(mem::replace(&mut op.rhs, empty_set()));
                }
            }
        }
    }