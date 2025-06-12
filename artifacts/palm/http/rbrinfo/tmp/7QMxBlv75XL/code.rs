fn remove_all_extra_values(&mut self, mut head: usize) {
        loop {
            let extra = self.remove_extra_value(head);

            if let Link::Extra(idx) = extra.next {
                head = idx;
            } else {
                break;
            }
        }
    }