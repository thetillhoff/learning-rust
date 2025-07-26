//! A dummy library 'b'.

// Declare the modules and make them public.
pub mod add;
pub mod sub;

/// Adds two to the given number.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = lib_b::add_two(arg);
///
/// assert_eq!(7, answer);
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn test_internal_add() {
        assert_eq!(5, add::add(2, 3));
    }

    #[test]
    fn test_internal_subtract() {
        assert_eq!(-1, sub::subtract(2, 3));
    }
}