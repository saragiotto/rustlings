// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

mod sausage_factory {
<<<<<<< Updated upstream
    fn make_sausage() {
||||||| Stash base
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
=======
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
>>>>>>> Stashed changes
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
