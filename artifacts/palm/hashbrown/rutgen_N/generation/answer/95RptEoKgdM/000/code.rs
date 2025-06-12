// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    
    struct MyKey;
    
    impl Equivalent<MyType> for MyKey {
        fn equivalent(&self, _: &MyType) -> bool {
            true
        }
    }
    
    struct MyType;

    #[test]
    fn test_equivalent_key() {
        let my_key = MyKey;
        let key_fn = equivalent_key(&my_key);
        
        let pair = (MyType, 42);
        
        assert!(key_fn(&pair));
    }
    
    #[test]
    fn test_equivalent_key_different_instance() {
        let my_key = MyKey;
        let key_fn = equivalent_key(&my_key);
        
        let pair1 = (MyType, 1);
        let pair2 = (MyType, 2);

        assert!(key_fn(&pair1));
        assert!(key_fn(&pair2));
    }
}

