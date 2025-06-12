fn visit_class<V: Visitor>(
        &mut self,
        ast: &'a ast::ClassBracketed,
        visitor: &mut V,
    ) -> Result<(), V::Err> {
        let mut ast = ClassInduct::from_bracketed(ast);
        loop {
            self.visit_class_pre(&ast, visitor)?;
            if let Some(x) = self.induct_class(&ast) {
                let child = x.child();
                self.stack_class.push((ast, x));
                ast = child;
                continue;
            }
            self.visit_class_post(&ast, visitor)?;

            // At this point, we now try to pop our call stack until it is
            // either empty or we hit another inductive case.
            loop {
                let (post_ast, frame) = match self.stack_class.pop() {
                    None => return Ok(()),
                    Some((post_ast, frame)) => (post_ast, frame),
                };
                // If this is a union or a binary op, then we might have
                // additional inductive steps to process.
                if let Some(x) = self.pop_class(frame) {
                    if let ClassFrame::BinaryRHS { ref op, .. } = x {
                        visitor.visit_class_set_binary_op_in(op)?;
                    }
                    ast = x.child();
                    self.stack_class.push((post_ast, x));
                    break;
                }
                // Otherwise, we've finished visiting all the child nodes for
                // this class node, so we can post visit it now.
                self.visit_class_post(&post_ast, visitor)?;
            }
        }
    }