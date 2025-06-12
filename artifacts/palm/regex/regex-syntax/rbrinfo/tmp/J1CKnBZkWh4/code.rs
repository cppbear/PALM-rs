fn push_class_op(
        &self,
        next_kind: ast::ClassSetBinaryOpKind,
        next_union: ast::ClassSetUnion,
    ) -> ast::ClassSetUnion {

        let item = ast::ClassSet::Item(next_union.into_item());
        let new_lhs = self.pop_class_op(item);
        self.parser().stack_class.borrow_mut().push(ClassState::Op {
            kind: next_kind,
            lhs: new_lhs,
        });
        ast::ClassSetUnion { span: self.span(), items: vec![] }
    }