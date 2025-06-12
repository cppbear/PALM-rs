// Answer 0

#[derive(Debug)]
struct PikeVM {
    slots_per_thread: usize,
    caps: Vec<Option<usize>>,
}

impl PikeVM {
    fn new(slots_per_thread: usize) -> Self {
        PikeVM {
            slots_per_thread,
            caps: vec![None; slots_per_thread * 2], // Initialize with twice the capacity for testing
        }
    }

    fn caps(&mut self, pc: usize) -> &mut [Option<usize>] {
        let i = pc * self.slots_per_thread;
        &mut self.caps[i..i + self.slots_per_thread]
    }
}

#[test]
fn test_caps_first_thread() {
    let mut vm = PikeVM::new(2);
    let result = vm.caps(0);
    assert_eq!(result.len(), 2);
    assert_eq!(result, &mut [None, None]);
}

#[test]
fn test_caps_second_thread() {
    let mut vm = PikeVM::new(2);
    let result = vm.caps(1);
    assert_eq!(result.len(), 2);
    assert_eq!(result, &mut [None, None]);
}

#[test]
fn test_caps_out_of_bounds() {
    let mut vm = PikeVM::new(2);
    let result = vm.caps(0);
    result[0] = Some(42);
    assert_eq!(vm.caps(1)[0], None); // Check that modifying one thread does not affect another
}

#[test]
#[should_panic]
fn test_caps_overflow() {
    let mut vm = PikeVM::new(2);
    // This should panic because we are trying to access beyond the allocated caps
    let _ = vm.caps(2);
}

