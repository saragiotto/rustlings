// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

fn main() {
    call_me(3);
}

<<<<<<< Updated upstream
fn call_me(num) {
||||||| Stash base
fn call_me(num:) {
=======
fn call_me(num: i32) {
>>>>>>> Stashed changes
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
