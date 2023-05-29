// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!

<<<<<<< Updated upstream
// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
||||||| Stash base
// I AM NOT DONE

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
=======
pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
>>>>>>> Stashed changes
}

<<<<<<< Updated upstream
fn main() {
    ("blue");
    ("red".to_string());
    (String::from("hi"));
    ("rust is fun!".to_owned());
    ("nice weather".into());
    (format!("Interpolation {}", "Station"));
    (&String::from("abc")[0..1]);
    ("  hello there ".trim());
    ("Happy Monday!".to_string().replace("Mon", "Tues"));
    ("mY sHiFt KeY iS sTiCkY".to_lowercase());
||||||| Stash base
mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: ???) -> ??? {
        // TODO: Complete the output declaration!
        let mut output: ??? = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
=======
mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            match command {
                Command::Uppercase => output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().to_string()),
                Command::Append(times) => output.push(string.clone() + &"bar".repeat(*times)),
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
>>>>>>> Stashed changes
}
