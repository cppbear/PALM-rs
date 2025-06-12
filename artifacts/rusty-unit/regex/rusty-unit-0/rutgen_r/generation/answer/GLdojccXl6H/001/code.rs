// Answer 0

#[derive(Debug)]
struct HirFrame {
    data: String,
}

struct CallStack {
    stack: Vec<HirFrame>,
}

impl CallStack {
    fn new() -> Self {
        CallStack { stack: Vec::new() }
    }
    
    fn trans(&self) -> &Self {
        self
    }
    
    fn push(&self, frame: HirFrame) {
        self.stack.push(frame);
    }
}

#[test]
fn test_push_with_non_empty_frame() {
    let mut call_stack = CallStack::new();
    let frame = HirFrame { data: "frame1".to_string() };
    
    call_stack.push(frame);
    
    assert_eq!(call_stack.stack.len(), 1);
}

#[test]
fn test_push_multiple_frames() {
    let mut call_stack = CallStack::new();
    
    for i in 0..5 {
        let frame = HirFrame { data: format!("frame{}", i) };
        call_stack.push(frame);
    }
    
    assert_eq!(call_stack.stack.len(), 5);
}

#[test]
fn test_push_empty_frame() {
    let mut call_stack = CallStack::new();
    let frame = HirFrame { data: "".to_string() };
    
    call_stack.push(frame);
    
    assert_eq!(call_stack.stack.len(), 1);
    assert_eq!(call_stack.stack[0].data, "");
}

#[test]
#[should_panic]
fn test_push_panics_when_frame_is_dropped() {
    let mut call_stack = CallStack::new();
    {
        let frame = HirFrame { data: "temp".to_string() };
        call_stack.push(frame);
    } // frame goes out of scope here
    
    // Attempt to push will now panic or lead to undefined behavior if we were tracking ownership improperly.
    call_stack.push(HirFrame { data: "new frame".to_string() });
}

