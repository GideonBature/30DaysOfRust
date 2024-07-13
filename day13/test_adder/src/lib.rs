pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add_three(a: i32) -> i32 {
    a + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        let result = add_two(5);
        assert_eq!(result, 7);
    }

    #[test]
    fn it_adds_three() {
        let result = add_three(-1);
        assert_eq!(result, 2);
    }
}
