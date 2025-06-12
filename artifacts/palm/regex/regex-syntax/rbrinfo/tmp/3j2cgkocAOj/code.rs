fn visit_class_set_binary_op_pre(
        &mut self,
        ast: &ast::ClassSetBinaryOp,
    ) -> Result<()> {
        self.increment_depth(&ast.span)
    }