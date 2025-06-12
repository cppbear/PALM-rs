// Answer 0

#[derive(Debug)]
struct HirFrame;

struct CallStack {
    stack: Vec<HirFrame>,
}

impl CallStack {
    fn new() -> Self {
        CallStack { stack: Vec::new() }
    }

    fn push(&mut self, frame: HirFrame) {
        self.stack.push(frame);
    }

    fn pop(&mut self) -> Option<HirFrame> {
        self.stack.pop()
    }
}

struct Translator {
    stack: std::cell::RefCell<CallStack>,
}

impl Translator {
    fn new() -> Self {
        Translator {
            stack: std::cell::RefCell::new(CallStack::new()),
        }
    }

    fn trans(&self) -> &CallStack {
        &self.stack.borrow_mut()
    }

    fn pop(&self) -> Option<HirFrame> {
        self.trans().pop()
    }
}

#[test]
fn test_pop_from_empty_stack() {
    let translator = Translator::new();
    let result = translator.pop();
    assert_eq!(result, None);
}

#[test]
fn test_pop_from_non_empty_stack() {
    let mut translator = Translator::new();
    let frame = HirFrame;
    translator.trans().push(frame);
    let result = translator.pop();
    assert!(result.is_some());
}

#[test]
fn test_multiple_pops() {
    let mut translator = Translator::new();
    let frame1 = HirFrame;
    let frame2 = HirFrame;
    translator.trans().push(frame1);
    translator.trans().push(frame2);
    
    let result1 = translator.pop();
    assert!(result1.is_some());
    
    let result2 = translator.pop();
    assert!(result2.is_some());
    
    let result3 = translator.pop();
    assert_eq!(result3, None);
}

