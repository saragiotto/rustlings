// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

<<<<<<< Updated upstream
// I AM NOT DONE
struct Wrapper {
    value: u32
||||||| Stash base
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

struct Wrapper {
    value: u32,
=======
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a hint.

struct Wrapper<T> {
    value: T,
>>>>>>> Stashed changes
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
<<<<<<< Updated upstream
        assert_eq!(Wrapper::new(42).value,  42);
||||||| Stash base
        assert_eq!(Wrapper::new(42).value, 42);
=======
        assert_eq!(Wrapper::<i32>::new(42).value, 42);
>>>>>>> Stashed changes
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::<&str>::new("Foo").value, "Foo");
    }
}