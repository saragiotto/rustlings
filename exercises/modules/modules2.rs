// modules2.rs
// Make me compile! Execute `rustlings hint modules2` for hints :)

mod delicious_snacks {
<<<<<<< Updated upstream
    use self::fruits::PEAR as fruit;
    use self::veggies::CUCUMBER as veggie;
||||||| Stash base
    // TODO: Fix these use statements
    use self::fruits::PEAR as ???
    use self::veggies::CUCUMBER as ???
=======
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;
>>>>>>> Stashed changes

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
