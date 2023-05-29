// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.
// You can put this right into the `println!` where the ??? is.
// Execute `rustlings hint primitive_types6` for hints!

<<<<<<< Updated upstream
// I AM NOT DONE

fn main() {
||||||| Stash base
// I AM NOT DONE

#[test]
fn indexing_tuple() {
=======
#[test]
fn indexing_tuple() {
>>>>>>> Stashed changes
    let numbers = (1, 2, 3);
<<<<<<< Updated upstream
    println!("The second number is {}", ???);
||||||| Stash base
    // Replace below ??? with the tuple indexing syntax.
    let second = ???;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
=======
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;

    assert_eq!(2, second, "This is not the 2nd number in the tuple!")
>>>>>>> Stashed changes
}
