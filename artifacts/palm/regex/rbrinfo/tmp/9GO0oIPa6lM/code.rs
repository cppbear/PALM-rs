pub fn compile(
        mut self,
        exprs: &[Hir],
    ) -> result::Result<Program, Error> {
        debug_assert!(exprs.len() >= 1);
        self.num_exprs = exprs.len();
        if exprs.len() == 1 {
            self.compile_one(&exprs[0])
        } else {
            self.compile_many(exprs)
        }
    }