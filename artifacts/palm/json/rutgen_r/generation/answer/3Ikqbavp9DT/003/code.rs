// Answer 0

#[test]
fn test_fmt_pos_int() {
    struct PosIntWrapper {
        n: N,
    }
    
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }
    
    let pos_int_value: u64 = 42;
    let wrapper = PosIntWrapper {
        n: N::PosInt(pos_int_value),
    };
    
    let mut buffer = std::fmt::Formatter::new();
    let result = wrapper.fmt(&mut buffer);
    
    assert!(result.is_ok());
    assert_eq!(buffer.to_string(), "42");
}

#[test]
fn test_fmt_neg_int() {
    struct NegIntWrapper {
        n: N,
    }
    
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }
    
    let neg_int_value: i64 = -42;
    let wrapper = NegIntWrapper {
        n: N::NegInt(neg_int_value),
    };
    
    let mut buffer = std::fmt::Formatter::new();
    let result = wrapper.fmt(&mut buffer);
    
    assert!(result.is_ok());
    assert_eq!(buffer.to_string(), "-42");
}

#[test]
fn test_fmt_float() {
    struct FloatWrapper {
        n: N,
    }
    
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }
    
    let float_value: f64 = 42.0;
    let wrapper = FloatWrapper {
        n: N::Float(float_value),
    };
    
    let mut buffer = std::fmt::Formatter::new();
    let result = wrapper.fmt(&mut buffer);
    
    assert!(result.is_ok());
    assert_eq!(buffer.to_string(), "42.0");
}

