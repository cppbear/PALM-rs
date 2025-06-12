fn visit_class_set_binary_op_post(
        &mut self,
        _ast: &ast::ClassSetBinaryOp,
    ) -> Result<()> {
        self.decrement_depth();
        Ok(())
    }