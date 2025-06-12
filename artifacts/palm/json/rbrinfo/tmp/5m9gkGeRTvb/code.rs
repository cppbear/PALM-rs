pub fn new(iter: I) -> LineColIterator<I> {
        LineColIterator {
            iter,
            line: 1,
            col: 0,
            start_of_line: 0,
        }
    }