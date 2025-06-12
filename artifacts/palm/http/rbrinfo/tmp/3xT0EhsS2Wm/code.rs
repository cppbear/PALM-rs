fn remove_extra_value<T>(
    mut raw_links: RawLinks<T>,
    extra_values: &mut Vec<ExtraValue<T>>,
    idx: usize,
) -> ExtraValue<T> {
    let prev;
    let next;

    {
        debug_assert!(extra_values.len() > idx);
        let extra = &extra_values[idx];
        prev = extra.prev;
        next = extra.next;
    }

    // First unlink the extra value
    match (prev, next) {
        (Link::Entry(prev), Link::Entry(next)) => {
            debug_assert_eq!(prev, next);

            raw_links[prev] = None;
        }
        (Link::Entry(prev), Link::Extra(next)) => {
            debug_assert!(raw_links[prev].is_some());

            raw_links[prev].as_mut().unwrap().next = next;

            debug_assert!(extra_values.len() > next);
            extra_values[next].prev = Link::Entry(prev);
        }
        (Link::Extra(prev), Link::Entry(next)) => {
            debug_assert!(raw_links[next].is_some());

            raw_links[next].as_mut().unwrap().tail = prev;

            debug_assert!(extra_values.len() > prev);
            extra_values[prev].next = Link::Entry(next);
        }
        (Link::Extra(prev), Link::Extra(next)) => {
            debug_assert!(extra_values.len() > next);
            debug_assert!(extra_values.len() > prev);

            extra_values[prev].next = Link::Extra(next);
            extra_values[next].prev = Link::Extra(prev);
        }
    }

    // Remove the extra value
    let mut extra = extra_values.swap_remove(idx);

    // This is the index of the value that was moved (possibly `extra`)
    let old_idx = extra_values.len();

    // Update the links
    if extra.prev == Link::Extra(old_idx) {
        extra.prev = Link::Extra(idx);
    }

    if extra.next == Link::Extra(old_idx) {
        extra.next = Link::Extra(idx);
    }

    // Check if another entry was displaced. If it was, then the links
    // need to be fixed.
    if idx != old_idx {
        let next;
        let prev;

        {
            debug_assert!(extra_values.len() > idx);
            let moved = &extra_values[idx];
            next = moved.next;
            prev = moved.prev;
        }

        // An entry was moved, we have to the links
        match prev {
            Link::Entry(entry_idx) => {
                // It is critical that we do not attempt to read the
                // header name or value as that memory may have been
                // "released" already.
                debug_assert!(raw_links[entry_idx].is_some());

                let links = raw_links[entry_idx].as_mut().unwrap();
                links.next = idx;
            }
            Link::Extra(extra_idx) => {
                debug_assert!(extra_values.len() > extra_idx);
                extra_values[extra_idx].next = Link::Extra(idx);
            }
        }

        match next {
            Link::Entry(entry_idx) => {
                debug_assert!(raw_links[entry_idx].is_some());

                let links = raw_links[entry_idx].as_mut().unwrap();
                links.tail = idx;
            }
            Link::Extra(extra_idx) => {
                debug_assert!(extra_values.len() > extra_idx);
                extra_values[extra_idx].prev = Link::Extra(idx);
            }
        }
    }

    debug_assert!({
        for v in &*extra_values {
            assert!(v.next != Link::Extra(old_idx));
            assert!(v.prev != Link::Extra(old_idx));
        }

        true
    });

    extra
}