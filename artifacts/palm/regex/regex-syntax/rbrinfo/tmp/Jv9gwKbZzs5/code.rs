fn pop_class_op(&self, rhs: ast::ClassSet) -> ast::ClassSet {
        let mut stack = self.parser().stack_class.borrow_mut();
        let (kind, lhs) = match stack.pop() {
            Some(ClassState::Op { kind, lhs }) => (kind, lhs),
            Some(state @ ClassState::Open { .. }) => {
                stack.push(state);
                return rhs;
            }
            None => unreachable!(),
        };
        let span = Span::new(lhs.span().start, rhs.span().end);
        ast::ClassSet::BinaryOp(ast::ClassSetBinaryOp {
            span: span,
            kind: kind,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }