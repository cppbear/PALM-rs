// Answer 0

#[test]
fn test_expectation_with_zero() {
    let mut formatter = std::fmt::Formatter::new();
    let phantom_data_visitor = PhantomDataVisitor::<T> { marker: PhantomData };
    phantom_data_visitor.expecting(&mut formatter);
}

#[test]
fn test_expectation_with_one() {
    let mut formatter = std::fmt::Formatter::new();
    let phantom_data_visitor = PhantomDataVisitor::<T> { marker: PhantomData };
    phantom_data_visitor.expecting(&mut formatter);
}

