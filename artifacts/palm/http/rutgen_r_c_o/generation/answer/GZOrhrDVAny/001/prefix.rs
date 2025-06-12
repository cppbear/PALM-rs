// Answer 0

#[test]
fn test_extensions_debug_none() {
    let extensions = Extensions { map: None };
    let _ = fmt(&extensions, &mut fmt::Formatter::new());
}

#[test]
fn test_extensions_debug_empty_map() {
    let extensions = Extensions {
        map: Some(Box::new(HashMap::new())),
    };
    let _ = fmt(&extensions, &mut fmt::Formatter::new());
}

#[test]
fn test_extensions_debug_single_i32() {
    let mut map = HashMap::new();
    map.insert(TypeId::of::<i32>(), Box::new(1) as Box<dyn AnyClone + Send + Sync>);
    let extensions = Extensions {
        map: Some(Box::new(map)),
    };
    let _ = fmt(&extensions, &mut fmt::Formatter::new());
}

#[test]
fn test_extensions_debug_single_string() {
    let mut map = HashMap::new();
    map.insert(TypeId::of::<String>(), Box::new("test".to_string()) as Box<dyn AnyClone + Send + Sync>);
    let extensions = Extensions {
        map: Some(Box::new(map)),
    };
    let _ = fmt(&extensions, &mut fmt::Formatter::new());
}

#[test]
fn test_extensions_debug_multiple_types() {
    let mut map = HashMap::new();
    map.insert(TypeId::of::<f64>(), Box::new(3.14) as Box<dyn AnyClone + Send + Sync>);
    map.insert(TypeId::of::<bool>(), Box::new(true) as Box<dyn AnyClone + Send + Sync>);
    let extensions = Extensions {
        map: Some(Box::new(map)),
    };
    let _ = fmt(&extensions, &mut fmt::Formatter::new());
}

