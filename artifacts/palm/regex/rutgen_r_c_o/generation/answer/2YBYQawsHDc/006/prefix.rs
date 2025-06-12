// Answer 0

#[test]
fn test_pop_with_repetition_frame() {
    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit(&mut self, _ast: &Ast) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let repetition = ast::Repetition; // Need to instantiate a repetition node.
    let frame = Frame::Repetition(&repetition);
    
    let visitor = HeapVisitor::new();
    let result = visitor.pop(frame);
}

#[test]
fn test_pop_with_group_frame() {
    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit(&mut self, _ast: &Ast) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let group = ast::Group; // Need to instantiate a group node.
    let frame = Frame::Group(&group);
    
    let visitor = HeapVisitor::new();
    let result = visitor.pop(frame);
} 

#[test]
fn test_pop_with_concat_frame_empty_tail() {
    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit(&mut self, _ast: &Ast) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let concat = ast::Concat {
        head: &Ast::Empty(ast::Span),
        tail: &[],
    };
    let frame = Frame::Concat {
        head: &concat.head,
        tail: &concat.tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop(frame);
}

#[test]
fn test_pop_with_concat_frame_non_empty_tail() {
    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit(&mut self, _ast: &Ast) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let tail = vec![Ast::Literal(ast::Literal), Ast::Dot(ast::Span)];
    let concat = ast::Concat {
        head: &tail[0],
        tail: &tail[1..],
    };
    let frame = Frame::Concat {
        head: &concat.head,
        tail: &concat.tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop(frame);
}

#[test]
fn test_pop_with_alternation_frame_empty_tail() {
    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit(&mut self, _ast: &Ast) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let alternation = ast::Alternation {
        head: &Ast::Empty(ast::Span),
        tail: &[],
    };
    let frame = Frame::Alternation {
        head: &alternation.head,
        tail: &alternation.tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop(frame);
}

#[test]
fn test_pop_with_alternation_frame_non_empty_tail() {
    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit(&mut self, _ast: &Ast) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let tail = vec![Ast::Literal(ast::Literal), Ast::Dot(ast::Span)];
    let alternation = ast::Alternation {
        head: &tail[0],
        tail: &tail[1..],
    };
    let frame = Frame::Alternation {
        head: &alternation.head,
        tail: &alternation.tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop(frame);
}

