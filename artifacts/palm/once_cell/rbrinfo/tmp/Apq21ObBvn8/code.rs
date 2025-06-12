pub fn get_mut(this: &mut Lazy<T, F>) -> Option<&mut T> {
            this.cell.get_mut()
        }