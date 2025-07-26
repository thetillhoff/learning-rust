//! A dummy library 'a'.

/// Adds one to the given number.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = lib_a::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_one() {
        assert_eq!(3, add_one(2));
    }
}