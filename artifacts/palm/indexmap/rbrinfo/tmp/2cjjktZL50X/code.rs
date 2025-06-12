pub(crate) fn new(map: &'a mut IndexMapCore<K, V>, index: usize) -> Self {
        Self {
            map: map.borrow_mut(),
            index,
        }
    }