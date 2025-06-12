pub fn force_mut(this: &mut Lazy<T, F>) -> &mut T {
            if this.cell.get_mut().is_none() {
                let value = match this.init.get_mut().take() {
                    Some(f) => f(),
                    None => panic!("Lazy instance has previously been poisoned"),
                };
                this.cell = OnceCell::with_value(value);
            }
            this.cell.get_mut().unwrap_or_else(|| unreachable!())
        }