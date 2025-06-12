fn visit<V: Visitor>(
        &mut self,
        mut ast: &'a Ast,
        mut visitor: V,
    ) -> Result<V::Output, V::Err> {
        self.stack.clear();
        self.stack_class.clear();

        visitor.start();
        loop {
            visitor.visit_pre(ast)?;
            if let Some(x) = self.induct(ast, &mut visitor)? {
                let child = x.child();
                self.stack.push((ast, x));
                ast = child;
                continue;
            }
            // No induction means we have a base case, so we can post visit
            // it now.
            visitor.visit_post(ast)?;

            // At this point, we now try to pop our call stack until it is
            // either empty or we hit another inductive case.
            loop {
                let (post_ast, frame) = match self.stack.pop() {
                    None => return visitor.finish(),
                    Some((post_ast, frame)) => (post_ast, frame),
                };
                // If this is a concat/alternate, then we might have additional
                // inductive steps to process.
                if let Some(x) = self.pop(frame) {
                    if let Frame::Alternation {..} = x {
                        visitor.visit_alternation_in()?;
                    }
                    ast = x.child();
                    self.stack.push((post_ast, x));
                    break;
                }
                // Otherwise, we've finished visiting all the child nodes for
                // this AST, so we can post visit it now.
                visitor.visit_post(post_ast)?;
            }
        }
    }