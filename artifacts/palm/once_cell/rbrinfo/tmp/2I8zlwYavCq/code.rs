fn compare_exchange(&self, value: &'a T) -> Result<(), *const T> {
        self.inner
            .compare_exchange(
                ptr::null_mut(),
                <*const T>::cast_mut(value),
                Ordering::Release,
                Ordering::Acquire,
            )
            .map(|_: *mut T| ())
            .map_err(<*mut T>::cast_const)
    }