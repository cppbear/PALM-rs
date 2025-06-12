pub fn try_with_capacity(capacity: usize) -> Result<HeaderMap<T>, MaxSizeReached> {
        if capacity == 0 {
            Ok(HeaderMap {
                mask: 0,
                indices: Box::new([]), // as a ZST, this doesn't actually allocate anything
                entries: Vec::new(),
                extra_values: Vec::new(),
                danger: Danger::Green,
            })
        } else {
            let raw_cap = match to_raw_capacity(capacity).checked_next_power_of_two() {
                Some(c) => c,
                None => return Err(MaxSizeReached { _priv: () }),
            };
            if raw_cap > MAX_SIZE {
                return Err(MaxSizeReached { _priv: () });
            }
            debug_assert!(raw_cap > 0);

            Ok(HeaderMap {
                mask: (raw_cap - 1) as Size,
                indices: vec![Pos::none(); raw_cap].into_boxed_slice(),
                entries: Vec::with_capacity(usable_capacity(raw_cap)),
                extra_values: Vec::new(),
                danger: Danger::Green,
            })
        }
    }