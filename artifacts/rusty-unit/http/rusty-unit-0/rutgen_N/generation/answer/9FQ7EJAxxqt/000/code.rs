// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Default)]
    struct Extensions {
        data: HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions {
                data: HashMap::new(),
            }
        }

        pub fn getOrInsertWith<T: Default + Clone + Send + Sync + 'static>(&mut self) -> &mut T {
            let key = std::any::type_name::<T>().to_string();
            self.data.entry(key).or_insert_with(|| Box::new(T::default()))
                .downcast_mut::<T>()
                .unwrap()
        }

        pub fn get<T: 'static>(&self) -> Option<&T> {
            let key = std::any::type_name::<T>().to_string();
            self.data.get(&key)?.downcast_ref::<T>()
        }
    }

    #[test]
    fn test_get_or_insert_default() {
        let mut ext = Extensions::new();
        *ext.get_or_insert_with::<i32>() += 2;

        assert_eq!(*ext.get::<i32>().unwrap(), 2);
    }

    #[test]
    fn test_get_or_insert_default_multiple_calls() {
        let mut ext = Extensions::new();
        *ext.get_or_insert_with::<i32>() += 3;
        *ext.get_or_insert_with::<i32>() += 4; // Should still point to the same value

        assert_eq!(*ext.get::<i32>().unwrap(), 7);
    }

    #[test]
    fn test_get_or_insert_default_different_types() {
        let mut ext = Extensions::new();
        *ext.get_or_insert_with::<i32>() += 1;
        *ext.get_or_insert_with::<String>() += "Hello";

        assert_eq!(*ext.get::<i32>().unwrap(), 1);
        assert_eq!(ext.get::<String>().unwrap(), "Hello");
    }
    
    #[test]
    fn test_insert_default_for_float() {
        let mut ext = Extensions::new();
        *ext.get_or_insert_with::<f64>() += 1.5;

        assert_eq!(*ext.get::<f64>().unwrap(), 1.5);
    }

    #[test]
    fn test_insert_default_for_empty_string() {
        let mut ext = Extensions::new();
        let default_str = ext.get_or_insert_with::<String>().clone();

        assert_eq!(&default_str, "");
    }
}

