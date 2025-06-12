// Answer 0

#[test]
fn test_is_f64_float() {
    #[derive(Copy, Clone)]
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }
    
    struct Number {
        n: N,
    }

    impl Number {
        pub fn is_f64(&self) -> bool {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::Float(_) => true,
                N::PosInt(_) | N::NegInt(_) => false,
            }
            #[cfg(feature = "arbitrary_precision")]
            {
                for c in self.n.chars() {
                    if c == '.' || c == 'e' || c == 'E' {
                        return self.n.parse::<f64>().ok().map_or(false, f64::is_finite);
                    }
                }
                false
            }
        }
    }

    let num_float = Number { n: N::Float(3.14) };
    assert_eq!(num_float.is_f64(), true);
}

#[test]
fn test_is_f64_negative_float() {
    #[derive(Copy, Clone)]
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }
    
    struct Number {
        n: N,
    }

    impl Number {
        pub fn is_f64(&self) -> bool {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::Float(_) => true,
                N::PosInt(_) | N::NegInt(_) => false,
            }
            #[cfg(feature = "arbitrary_precision")]
            {
                for c in self.n.chars() {
                    if c == '.' || c == 'e' || c == 'E' {
                        return self.n.parse::<f64>().ok().map_or(false, f64::is_finite);
                    }
                }
                false
            }
        }
    }

    let num_float = Number { n: N::Float(-2.71) };
    assert_eq!(num_float.is_f64(), true);
}

#[test]
fn test_is_f64_positive_int() {
    #[derive(Copy, Clone)]
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }
    
    struct Number {
        n: N,
    }

    impl Number {
        pub fn is_f64(&self) -> bool {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::Float(_) => true,
                N::PosInt(_) | N::NegInt(_) => false,
            }
            #[cfg(feature = "arbitrary_precision")]
            {
                for c in self.n.chars() {
                    if c == '.' || c == 'e' || c == 'E' {
                        return self.n.parse::<f64>().ok().map_or(false, f64::is_finite);
                    }
                }
                false
            }
        }
    }

    let num_int = Number { n: N::PosInt(5) }; 
    assert_eq!(num_int.is_f64(), false);
}

#[test]
fn test_is_f64_negative_int() {
    #[derive(Copy, Clone)]
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }
    
    struct Number {
        n: N,
    }

    impl Number {
        pub fn is_f64(&self) -> bool {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::Float(_) => true,
                N::PosInt(_) | N::NegInt(_) => false,
            }
            #[cfg(feature = "arbitrary_precision")]
            {
                for c in self.n.chars() {
                    if c == '.' || c == 'e' || c == 'E' {
                        return self.n.parse::<f64>().ok().map_or(false, f64::is_finite);
                    }
                }
                false
            }
        }
    }

    let num_int = Number { n: N::NegInt(-3) }; 
    assert_eq!(num_int.is_f64(), false);
}

