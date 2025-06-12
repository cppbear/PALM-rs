pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in 1..slice.len() {
            slice.swap(i, self.usize(..=i));
        }
    }