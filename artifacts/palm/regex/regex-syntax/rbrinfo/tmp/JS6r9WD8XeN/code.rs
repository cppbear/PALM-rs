fn push_class_open(
        &self,
        parent_union: ast::ClassSetUnion,
    ) -> Result<ast::ClassSetUnion> {
        assert_eq!(self.char(), '[');

        let (nested_set, nested_union) = self.parse_set_class_open()?;
        self.parser().stack_class.borrow_mut().push(ClassState::Open {
            union: parent_union,
            set: nested_set,
        });
        Ok(nested_union)
    }