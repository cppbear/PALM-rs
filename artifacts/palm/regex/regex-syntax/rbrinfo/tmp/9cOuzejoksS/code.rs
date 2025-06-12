fn visit_class_pre<V: Visitor>(
        &self,
        ast: &ClassInduct<'a>,
        visitor: &mut V,
    ) -> Result<(), V::Err> {
        match *ast {
            ClassInduct::Item(item) => {
                visitor.visit_class_set_item_pre(item)?;
            }
            ClassInduct::BinaryOp(op) => {
                visitor.visit_class_set_binary_op_pre(op)?;
            }
        }
        Ok(())
    }