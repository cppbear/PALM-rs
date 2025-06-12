fn visit_class_post<V: Visitor>(
        &self,
        ast: &ClassInduct<'a>,
        visitor: &mut V,
    ) -> Result<(), V::Err> {
        match *ast {
            ClassInduct::Item(item) => {
                visitor.visit_class_set_item_post(item)?;
            }
            ClassInduct::BinaryOp(op) => {
                visitor.visit_class_set_binary_op_post(op)?;
            }
        }
        Ok(())
    }