// traits1.rs
// Time to implement some traits!
// 
// Your task is to implement the trait
// `AppendBar' for the type `String'.
// 
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.

<<<<<<< Updated upstream
// I AM NOT DONE
||||||| Stash base
// I AM NOT DONE

=======
>>>>>>> Stashed changes
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
<<<<<<< Updated upstream
    //Add your code here

||||||| Stash base
    // TODO: Implement `AppendBar` for type `String`.
=======
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(self) -> Self {
        self + "Bar"
    }
>>>>>>> Stashed changes
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_FooBar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_BarBar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }

}