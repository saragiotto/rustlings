// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
<<<<<<< Updated upstream
    let number = "3"; // don't change this line
    println!("Number {}", number);
    number = 3;
    println!("Number {}", number);
||||||| Stash base
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
=======
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    {
        let number = 3; // don't rename this variable
        println!("Number plus two is : {}", number + 2);
    }
>>>>>>> Stashed changes
}
