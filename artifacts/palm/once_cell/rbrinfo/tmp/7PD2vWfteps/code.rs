fn drop(&mut self) {
        let queue = self.queue.swap(self.new_queue, Ordering::AcqRel);

        let state = strict::addr(queue) & STATE_MASK;
        assert_eq!(state, RUNNING);

        unsafe {
            let mut waiter = strict::map_addr(queue, |q| q & !STATE_MASK);
            while !waiter.is_null() {
                let next = (*waiter).next;
                let thread = (*waiter).thread.take().unwrap();
                (*waiter).signaled.store(true, Ordering::Release);
                waiter = next;
                thread.unpark();
            }
        }
    }