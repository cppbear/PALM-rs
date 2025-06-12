fn visit<V: Visitor>(
        &mut self,
        mut hir: &'a Hir,
        mut visitor: V,
    ) -> Result<V::Output, V::Err> {
        self.stack.clear();

        visitor.start();
        loop {
            visitor.visit_pre(hir)?;
            if let Some(x) = self.induct(hir) {
                let child = x.child();
                self.stack.push((hir, x));
                hir = child;
                continue;
            }
            // No induction means we have a base case, so we can post visit
            // it now.
            visitor.visit_post(hir)?;

            // At this point, we now try to pop our call stack until it is
            // either empty or we hit another inductive case.
            loop {
                let (post_hir, frame) = match self.stack.pop() {
                    None => return visitor.finish(),
                    Some((post_hir, frame)) => (post_hir, frame),
                };
                // If this is a concat/alternate, then we might have additional
                // inductive steps to process.
                if let Some(x) = self.pop(frame) {
                    if let Frame::Alternation {..} = x {
                        visitor.visit_alternation_in()?;
                    }
                    hir = x.child();
                    self.stack.push((post_hir, x));
                    break;
                }
                // Otherwise, we've finished visiting all the child nodes for
                // this HIR, so we can post visit it now.
                visitor.visit_post(post_hir)?;
            }
        }
    }