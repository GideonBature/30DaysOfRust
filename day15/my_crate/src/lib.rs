//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds two numbers given.
///
/// # Examples
///
/// ```
/// let arg_left = 5;
/// let arg_right = 1;
/// let answer = my_crate::add(arg_left, arg_right);
///
/// assert_eq!(6, answer);
/// ```

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
