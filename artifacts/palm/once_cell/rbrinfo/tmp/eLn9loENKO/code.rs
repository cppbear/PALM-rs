fn init<E>(&self, f: impl FnOnce() -> Result<Box<T>, E>) -> Result<&T, E> {
            let val = f()?;
            let mut ptr = Box::into_raw(val);
            let exchange = self.inner.compare_exchange(
                ptr::null_mut(),
                ptr,
                Ordering::Release,
                Ordering::Acquire,
            );
            if let Err(old) = exchange {
                drop(unsafe { Box::from_raw(ptr) });
                ptr = old;
            }
            Ok(unsafe { &*ptr })
        }