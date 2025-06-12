fn wait(queue: &AtomicPtr<Waiter>, mut curr_queue: *mut Waiter) {
    let curr_state = strict::addr(curr_queue) & STATE_MASK;
    loop {
        let node = Waiter {
            thread: Cell::new(Some(thread::current())),
            signaled: AtomicBool::new(false),
            next: strict::map_addr(curr_queue, |q| q & !STATE_MASK),
        };
        let me = &node as *const Waiter as *mut Waiter;

        let exchange = queue.compare_exchange(
            curr_queue,
            strict::map_addr(me, |q| q | curr_state),
            Ordering::Release,
            Ordering::Relaxed,
        );
        if let Err(new_queue) = exchange {
            if strict::addr(new_queue) & STATE_MASK != curr_state {
                return;
            }
            curr_queue = new_queue;
            continue;
        }

        while !node.signaled.load(Ordering::Acquire) {
            thread::park();
        }
        break;
    }
}