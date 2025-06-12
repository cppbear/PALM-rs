// Answer 0

#[test]
fn test_bucket_read() {
    struct TestType {
        value: i32,
    }

    let test_value = TestType { value: 42 };
    let layout = Layout::new::<TestType>();
    unsafe {
        let ptr = Global.alloc(layout).unwrap().cast::<TestType>();
        ptr.write(test_value);
        let bucket = Bucket { ptr: NonNull::new_unchecked(ptr) };

        let read_value = bucket.read();
        assert_eq!(read_value.value, 42);

        // Clean up
        Global.dealloc(ptr, layout);
    }
}

#[test]
#[should_panic]
fn test_bucket_read_panic_on_scope_exit() {
    struct TestType {
        value: i32,
    }

    let layout = Layout::new::<TestType>();
    unsafe {
        let ptr = Global.alloc(layout).unwrap().cast::<TestType>();
        ptr.write(TestType { value: 42 });
        
        let bucket = Bucket { ptr: NonNull::new_unchecked(ptr) };
        
        // Reading the value and causing the destructor to panic when it goes out of scope
        let _panic_read = bucket.read();
        
        // Clean up
        Global.dealloc(ptr, layout);
    }
}

