pub fn take(&mut self) -> Value {
        mem::replace(self, Value::Null)
    }