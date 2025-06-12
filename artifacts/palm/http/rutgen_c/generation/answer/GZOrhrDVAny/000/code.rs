// Answer 0

#[test]
fn test_extensions_debug_impl() {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;
    use std::fmt;
    use std::hash::{BuildHasherDefault, Hasher};

    type AnyMap = HashMap<
        TypeId,
        Box<dyn Any + Send + Sync>,
        BuildHasherDefault<IdHasher>,
    >;

    #[derive(Default)]
    struct Extensions {
        map: Option<Box<AnyMap>>,
    }

    impl fmt::Debug for Extensions {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Extensions").finish()
        }
    }

    // Create an instance of Extensions
    let extensions = Extensions::default();
    
    // Capture the output of the debug implementation
    let debug_output = format!("{:?}", extensions);
    
    // Check if the output is as expected
    assert_eq!(debug_output, "Extensions");
}

#[test]
fn test_extensions_debug_empty() {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;
    use std::fmt;
    use std::hash::{BuildHasherDefault, Hasher};

    type AnyMap = HashMap<
        TypeId,
        Box<dyn Any + Send + Sync>,
        BuildHasherDefault<IdHasher>,
    >;

    #[derive(Default)]
    struct Extensions {
        map: Option<Box<AnyMap>>,
    }

    impl fmt::Debug for Extensions {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Extensions").finish()
        }
    }

    // Create another instance of Extensions
    let empty_extensions = Extensions::default();
    
    // Directly asserting debug, as it's known to return the same for empty.
    assert_eq!(format!("{:?}", empty_extensions), "Extensions");
}

