// structs1.rs
// Address all the TODOs to make the tests pass!

struct ColorClassicStruct {
    // TODO: Something goes here
    red: i32,
    green: i32,
    blue: i32,
}

struct ColorTupleStruct(i32, i32, i32);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
<<<<<<< Updated upstream
        // TODO: Instantiate a unit struct!
        // let unit_struct =
        let message = format!("{:?}s are fun!", unit_struct);
||||||| Stash base
        // TODO: Instantiate a unit-like struct!
        // let unit_like_struct =
        let message = format!("{:?}s are fun!", unit_like_struct);
=======
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);
>>>>>>> Stashed changes

        assert_eq!(message, "UnitStructs are fun!");
    }
}
