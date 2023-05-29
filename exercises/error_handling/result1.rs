// result1.rs
// Make this test pass! Execute `rustlings hint result1` for hints :)

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
<<<<<<< Updated upstream:exercises/error_handling/result1.rs
        Ok(PositiveNonzeroInteger(value as u64))
||||||| Stash base:exercises/error_handling/errors4.rs
        // Hmm...? Why is this only returning an Ok value?
        Ok(PositiveNonzeroInteger(value as u64))
=======
        // Hmm...? Why is this only returning an Ok value?
        match value {
            y if y < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            _ => Ok(PositiveNonzeroInteger(value as u64)),
        }
>>>>>>> Stashed changes:exercises/error_handling/errors4.rs
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
