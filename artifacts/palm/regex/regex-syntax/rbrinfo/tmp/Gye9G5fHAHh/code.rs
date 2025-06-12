fn visit_class_set_binary_op_post(
        &mut self,
        op: &ast::ClassSetBinaryOp,
    ) -> Result<()> {
        use ast::ClassSetBinaryOpKind::*;

        if self.flags().unicode() {
            let mut rhs = self.pop().unwrap().unwrap_class_unicode();
            let mut lhs = self.pop().unwrap().unwrap_class_unicode();
            let mut cls = self.pop().unwrap().unwrap_class_unicode();
            if self.flags().case_insensitive() {
                rhs.case_fold_simple();
                lhs.case_fold_simple();
            }
            match op.kind {
                Intersection => lhs.intersect(&rhs),
                Difference => lhs.difference(&rhs),
                SymmetricDifference => lhs.symmetric_difference(&rhs),
            }
            cls.union(&lhs);
            self.push(HirFrame::ClassUnicode(cls));
        } else {
            let mut rhs = self.pop().unwrap().unwrap_class_bytes();
            let mut lhs = self.pop().unwrap().unwrap_class_bytes();
            let mut cls = self.pop().unwrap().unwrap_class_bytes();
            if self.flags().case_insensitive() {
                rhs.case_fold_simple();
                lhs.case_fold_simple();
            }
            match op.kind {
                Intersection => lhs.intersect(&rhs),
                Difference => lhs.difference(&rhs),
                SymmetricDifference => lhs.symmetric_difference(&rhs),
            }
            cls.union(&lhs);
            self.push(HirFrame::ClassBytes(cls));
        }
        Ok(())
    }