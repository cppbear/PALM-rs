// Answer 0

#[test]
fn test_caps_valid_input() {
    let mut threads = Threads::new();
    threads.slots_per_thread = 2;
    threads.caps = vec![Some(1), Some(2), Some(3), Some(4)];
    let pc = 1;
    let result = threads.caps(pc);
}

#[test]
#[should_panic]
fn test_caps_out_of_bounds_low() {
    let mut threads = Threads::new();
    threads.slots_per_thread = 2;
    threads.caps = vec![Some(1), Some(2)];
    let pc = 0;
    let result = threads.caps(pc);
}

#[test]
#[should_panic]
fn test_caps_out_of_bounds_high() {
    let mut threads = Threads::new();
    threads.slots_per_thread = 1;
    threads.caps = vec![Some(1)];
    let pc = 2;
    let result = threads.caps(pc);
}

#[test]
#[should_panic]
fn test_caps_zero_slots_per_thread() {
    let mut threads = Threads::new();
    threads.slots_per_thread = 0;
    threads.caps = vec![Some(1), Some(2), Some(3)];
    let pc = 0;
    let result = threads.caps(pc);
}

#[test]
fn test_caps_multiple_slots() {
    let mut threads = Threads::new();
    threads.slots_per_thread = 3;
    threads.caps = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)];
    let pc = 1;
    let result = threads.caps(pc);
}

