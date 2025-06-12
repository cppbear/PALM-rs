pub fn get(this: &Lazy<T, F>) -> Option<&T> {
            this.cell.get()
        }