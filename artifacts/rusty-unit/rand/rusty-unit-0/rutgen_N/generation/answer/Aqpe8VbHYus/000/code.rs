// Answer 0

#[test]
fn test_new_non_empty_slice() {
    struct Choose<'a, T> {
        slice: &'a [T],
        range: std::ops::Range<usize>,
        num_choices: std::num::NonZeroUsize,
    }
    
    enum Empty {}

    fn new<T>(slice: &'_ [T]) -> Result<Choose<T>, Empty> {
        let num_choices = std::num::NonZeroUsize::new(slice.len()).ok_or(Empty)?;

        Ok(Choose {
            slice,
            range: 0..num_choices.get(),
            num_choices,
        })
    }

    let slice = [1, 2, 3];
    let result = new(&slice);
    assert!(result.is_ok());
}

#[test]
fn test_new_empty_slice() {
    struct Choose<'a, T> {
        slice: &'a [T],
        range: std::ops::Range<usize>,
        num_choices: std::num::NonZeroUsize,
    }

    enum Empty {}

    fn new<T>(slice: &'_ [T]) -> Result<Choose<T>, Empty> {
        let num_choices = std::num::NonZeroUsize::new(slice.len()).ok_or(Empty)?;

        Ok(Choose {
            slice,
            range: 0..num_choices.get(),
            num_choices,
        })
    }

    let slice: &[i32] = &[];
    let result = new(&slice);
    assert!(result.is_err());
}

