fn fmt_class_set_binary_op_kind(
        &mut self,
        ast: &ast::ClassSetBinaryOpKind,
    ) -> fmt::Result {
        use ast::ClassSetBinaryOpKind::*;
        match *ast {
            Intersection => self.wtr.write_str("&&"),
            Difference => self.wtr.write_str("--"),
            SymmetricDifference => self.wtr.write_str("~~"),
        }
    }