fn initialize_or_wait(queue: &AtomicPtr<Waiter>, mut init: Option<&mut dyn FnMut() -> bool>) {
    let mut curr_queue = queue.load(Ordering::Acquire);

    loop {
        let curr_state = strict::addr(curr_queue) & STATE_MASK;
        match (curr_state, &mut init) {
            (COMPLETE, _) => return,
            (INCOMPLETE, Some(init)) => {
                let exchange = queue.compare_exchange(
                    curr_queue,
                    strict::map_addr(curr_queue, |q| (q & !STATE_MASK) | RUNNING),
                    Ordering::Acquire,
                    Ordering::Acquire,
                );
                if let Err(new_queue) = exchange {
                    curr_queue = new_queue;
                    continue;
                }
                let mut guard = Guard { queue, new_queue: INCOMPLETE_PTR };
                if init() {
                    guard.new_queue = COMPLETE_PTR;
                }
                return;
            }
            (INCOMPLETE, None) | (RUNNING, _) => {
                wait(queue, curr_queue);
                curr_queue = queue.load(Ordering::Acquire);
            }
            _ => debug_assert!(false),
        }
    }
}