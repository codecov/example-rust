const EYES: &'static str = ":";

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
    }
    else if face == frown() {
        "Frown"
    }
    else if face == angry() {
        "Angry"
    }
    else {
        "I don't know"
    }
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
}

