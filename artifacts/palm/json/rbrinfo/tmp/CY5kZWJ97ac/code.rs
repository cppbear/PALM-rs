pub fn sort_all_objects(&mut self) {
        #[cfg(feature = "preserve_order")]
        {
            match self {
                Value::Object(map) => {
                    map.sort_keys();
                    map.values_mut().for_each(Value::sort_all_objects);
                }
                Value::Array(list) => {
                    list.iter_mut().for_each(Value::sort_all_objects);
                }
                _ => {}
            }
        }
    }