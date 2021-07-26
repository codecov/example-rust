use std::fs::File;
use std::io;
use std::io::Read;
use std::str;


const EYES: &str = ":";

pub fn smile() -> String {
    format!("{}{}", EYES, ")")
}

pub fn frown() -> String {
    format!("{}{}", EYES, "(")
}

pub fn angry() -> String {
    format!("{}{}{}", ">", EYES, "(")
}

/// Provides a string representation of a face
///
/// # Examples
///
/// ```
/// # use examplerust::*;
/// assert_eq!(which(&frown()), "Frown");
/// ```
pub fn which(face: &str) -> &'static str {
    if face == smile() {
        "Smile"
    } else if face == frown() {
        "Frown"
    } else if face == angry() {
        "Angry"
    } else {
        "I don't know"
    }
}

/// This function is not called during tests, so it will be considered dead code.
/// By default, and because of dead-code elimination it won't be reported as uncovered
/// since the function will be removed from executable.
/// This is accounted for in the Travis configuration by passing the compiler flag
/// `-C link-dead-code` when building the tests. This flag disables dead code
/// elimination and allows this function to be reported correctly.
pub fn not_called() {
    println!("This is dead code");
    unreachable!();
}

pub fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
    let f = File::open(file_name);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


pub fn read_username_from_file2(file_name: &str) -> Result<String, io::Error> {
    let mut f = File::open(file_name)?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_smile() {
        assert_eq!(smile(), ":)");
    }

    #[test]
    fn can_frown() {
        assert_eq!(frown(), ":(");
    }

    #[test]
    fn can_angry() {
        assert_eq!(angry(), ">:(");
    }

    #[test]
    fn string_representation() {
        assert_eq!(which(&smile()), "Smile");
    }

    #[test]
    fn read_username_should_pass() {
        let username = read_username_from_file("username.txt").unwrap();
        assert_eq!(username, "alex");
    }

//    #[test]
//    fn read_username_should_fail() {
//        let username = read_username_from_file("non-existing.txt");
//        assert_eq!(username.is_err(), true);
//    }


    #[test]
    fn read_username2_should_pass() {
        let username = read_username_from_file2("username.txt").unwrap();
        assert_eq!(username, "alex");
    }

//    #[test]
//    fn read_username2_should_fail() {
//        let username = read_username_from_file2("non-existing.txt");
//        assert_eq!(username.is_err(), true);
//    }
}
