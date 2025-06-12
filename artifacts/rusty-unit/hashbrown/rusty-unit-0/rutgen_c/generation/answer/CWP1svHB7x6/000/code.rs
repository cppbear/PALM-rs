// Answer 0

#[test]
fn test_into_iter_fmt_empty() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let iter = IntoIter::<i32, TestAllocator> {
        iter: map::IntoIter::new(),
    };
    let mut output = Vec::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    
    let result = iter.fmt(formatter);
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "[]");
}

#[test]
fn test_into_iter_fmt_single_element() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map = HashMap::new();
    map.insert(1, "one");
    
    let iter = IntoIter::<i32, &str, TestAllocator> {
        iter: map.into_iter(),
    };
    let mut output = Vec::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    
    let result = iter.fmt(formatter);
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "[1]");
}

#[test]
fn test_into_iter_fmt_multiple_elements() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    
    let iter = IntoIter::<i32, &str, TestAllocator> {
        iter: map.into_iter(),
    };
    let mut output = Vec::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    
    let result = iter.fmt(formatter);
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "[1, 2]");
}

