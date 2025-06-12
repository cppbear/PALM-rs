// Answer 0

#[test]
fn test_pop_frame_concat_tail_empty() {
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let empty_tail: &[Hir] = &[];
    let frame = Frame::Concat {
        head: &Hir { kind: HirKind::Empty, info: HirInfo::default() }, 
        tail: empty_tail,
    };
    
    let visitor = DummyVisitor;
    let visitor_output = visitor.visit(&frame);
    
    let heap_visitor = HeapVisitor::new();
    let result = heap_visitor.pop(frame);
}

#[test]
fn test_pop_frame_alternation_tail_empty() {
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let empty_tail: &[Hir] = &[];
    let frame = Frame::Alternation {
        head: &Hir { kind: HirKind::Empty, info: HirInfo::default() }, 
        tail: empty_tail,
    };
    
    let visitor = DummyVisitor;
    let visitor_output = visitor.visit(&frame);
    
    let heap_visitor = HeapVisitor::new();
    let result = heap_visitor.pop(frame);
}

