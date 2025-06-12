pub fn into_value(this: Lazy<T, F>) -> Result<T, F> {
            let cell = this.cell;
            let init = this.init;
            cell.into_inner().ok_or_else(|| {
                init.take().unwrap_or_else(|| panic!("Lazy instance has previously been poisoned"))
            })
        }